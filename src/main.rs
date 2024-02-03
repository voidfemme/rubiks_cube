mod graphics;

use crate::graphics::vertex::Vertex;
use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};

use futures::executor::block_on;

async fn setup_wgpu(
    instance: &wgpu::Instance,
    surface: &wgpu::Surface<'_>,
    window: &winit::window::Window,
) -> (wgpu::Device, wgpu::Queue, wgpu::SurfaceConfiguration) {
    // Request an adapter. This represents a physical or virtual GPU.
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        })
        .await
        .expect("Failed to find an appropriate adapter");

    let surface_capabilities = surface.get_capabilities(&adapter);
    let preferred_format = surface_capabilities.formats[0];

    // The concept of SwapChain has been replaced with direct surface management in newer versions
    // of wgpu. Therefore, we manage the presentation of frames directly to the surface
    let config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: preferred_format,
        width: window.inner_size().width,
        height: window.inner_size().height,
        present_mode: wgpu::PresentMode::Fifo, // Use vsync.
        alpha_mode: wgpu::CompositeAlphaMode::Auto, // Example value, adjust as needed.
        view_formats: vec![],                  // Example empty vec, adjust as needed.
        desired_maximum_frame_latency: 2,      // Example value, adjust as needed.
    };

    // Request a device and command queue from the adapter.
    let (device, queue) = adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::empty(), // Adjust as needed
                required_limits: wgpu::Limits::default(),
            },
            None,
        )
        .await
        .expect("Failed to create device");

    surface.configure(&device, &config);
    (device, queue, config)
}

async fn setup_render_pipeline(
    device: &wgpu::Device,
    config: &wgpu::SurfaceConfiguration,
) -> wgpu::RenderPipeline {
    // Load the shaders
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("Shader"),
        source: wgpu::ShaderSource::Wgsl(include_str!("graphics/shaders.wgsl").into()),
    });

    // Create the render pipeline
    let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("Render Pipeline Layout"),
        bind_group_layouts: &[],
        push_constant_ranges: &[],
    });

    let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("Render Pipeline"),
        layout: Some(&render_pipeline_layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: "vs_main",
            buffers: &[Vertex::desc()],
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: "fs_main",
            targets: &[Some(wgpu::ColorTargetState {
                format: config.format,
                blend: Some(wgpu::BlendState::REPLACE),
                write_mask: wgpu::ColorWrites::ALL,
            })],
        }),
        primitive: wgpu::PrimitiveState::default(),
        depth_stencil: None,
        multisample: wgpu::MultisampleState::default(),
        multiview: None,
    });

    render_pipeline
}

fn main() {
    let event_loop = EventLoop::new().expect("Failed to create event loop"); // returns -> Result<EventLoop<()>, EventLoopError>
    let window = WindowBuilder::new()
        .with_title("Rubik's Cube Simulator")
        .build(&event_loop)
        .unwrap();

    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::all(),
        flags: wgpu::InstanceFlags::empty(),
        dx12_shader_compiler: wgpu::Dx12Compiler::default(),
        gles_minor_version: wgpu::Gles3MinorVersion::default(),
        ..Default::default()
    });
    let surface_result = instance.create_surface(&window);

    let surface = match surface_result {
        Ok(surface) => surface,
        Err(e) => {
            panic!("Failed to create surface: {:?}", e);
        }
    };

    // Since setup_wgpu is async, consider using block_on here or initializing wgpu resources
    // synchronously if possible
    let (device, _queue, config) = block_on(setup_wgpu(&instance, &surface, &window));
    let render_pipeline = block_on(setup_render_pipeline(&device, &config));

    let _ = event_loop.run(move |event, event_loop_window_target| match event {
        Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } => {
            event_loop_window_target.exit();
        }
        Event::WindowEvent {
            event: WindowEvent::RedrawRequested,
            ..
        } => {
            let frame = surface
                .get_current_texture()
                .expect("Failed to acquire next swap chain texture");
            let view = frame
                .texture
                .create_view(&wgpu::TextureViewDescriptor::default());
            let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

            {
                let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("Render Pass"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color {
                                r: 0.0,
                                g: 0.0,
                                b: 0.0,
                                a: 1.0,
                            }),
                            store: wgpu::StoreOp::Store,
                        },
                    })],
                    depth_stencil_attachment: None,
                    timestamp_writes: None,
                    occlusion_query_set: None,
                });

                render_pass.set_pipeline(&render_pipeline);
            }
        }
        _ => {}
    });
}
