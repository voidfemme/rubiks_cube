/* Two types of shaders:
*   - vertex shaders 
*   - fragment (or pixel) shaders
*
* Define Shader's Purpose:
*   - Vertex Shader: Determines the position of vertices in your scene.
*       - For a cube, this shader will project the cube's vertices onto a 2D viewport based on the
*       cube's position, the camera's position, and projection parameters.
*       - Fragment Shader: Determines the color of each pixel in the rendered output. Includes the
*       logic for lighting, texturing and color calculations. For a Rubik's Cube, you might
*       initially use solid colors for each face.
**/
