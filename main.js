"use strict";

window.addEventListener("load", setupWebGL, false);

/** @type WebGLRenderingContextBase */
let gl;
let program;
let u_mouse;
let a_pos;
let text;

function setupWebGL(evt) {
    window.removeEventListener(evt.type, setupWebGL, false);
    if (!(gl = getRenderingContext())) return;

    let source = document.querySelector("#vertex-shader").innerHTML;
    const vertexShader = gl.createShader(gl.VERTEX_SHADER);
    gl.shaderSource(vertexShader, source);
    gl.compileShader(vertexShader);

    source = document.querySelector("#fragment-shader").innerHTML;
    const fragmentShader = gl.createShader(gl.FRAGMENT_SHADER);
    gl.shaderSource(fragmentShader, source);
    gl.compileShader(fragmentShader);

    console.log(gl.getShaderInfoLog(fragmentShader));

    program = gl.createProgram();
    gl.attachShader(program, vertexShader);
    gl.attachShader(program, fragmentShader);
    console.log(gl.getShaderInfoLog(vertexShader));

    gl.linkProgram(program);

    /*
    gl.detachShader(program, vertexShader);
    gl.detachShader(program, fragmentShader);
    gl.deleteShader(vertexShader);
    gl.deleteShader(fragmentShader);
    /**/

    if (!gl.getProgramParameter(program, gl.LINK_STATUS)) {
        const linkErrLog = gl.getProgramInfoLog(program);
        console.log(linkErrLog);
        cleanup();
        text.textContent = `Shader program did not link successfully. Error log: ${linkErrLog}`;
        return;
    }

    u_mouse = gl.getUniformLocation(program, 'u_mouse');

    gl.useProgram(program);
    a_pos = gl.getAttribLocation(program, 'a_pos');

    initializeAttributes();

    render();

}

function render() {
    gl.viewport(0, 0, gl.viewportWidth, gl.viewportHeight)
    gl.clearColor(0, 0, 0, 1);

    gl.vertexAttribPointer(a_pos, buffer.itemSize, gl.FLOAT, false, 0, 0);
    gl.enableVertexAttribArray(a_pos);

    gl.drawArrays(gl.TRIANGLES, 0, buffer.numberOfItems);
}

function mousemove(/** @type MouseEvent */ evt) {
    let [x, y] = [evt.offsetX / 512 - 1, 1 - evt.offsetY / 512];

    gl.uniform2f(u_mouse, x, y);
    text.innerText = `${x}\n${y}`;
    render();
}

let buffer;
function initializeAttributes() {
    //gl.enableVertexAttribArray(0);
    buffer = gl.createBuffer();
    gl.bindBuffer(gl.ARRAY_BUFFER, buffer);

    /*
    gl.vertexAttribPointer(0, 1, gl.FLOAT, false, 0, 0);

    // triangle 0
    gl.vertexAttrib3f(0, 0, 0, 0);      // lower left (quadrant 1)
    gl.vertexAttrib3f(1, 0, 1, 0);      // upper left
    gl.vertexAttrib3f(2, 0, 0, 0);      // upper right

    // triangle 1
    gl.vertexAttrib3f(3, 0, 0, 0);      // lower left
    gl.vertexAttrib3f(4, 1, 1, 0);      // upper right
    gl.vertexAttrib3f(5, 1, 0, 0);      // lower right
    */

    const triangleVertices = [
        -1, -1, 0, // LL
        1, -1, 0, // LR
        -1, 1, 0, // UL

        1, 1, 0, // LL
        1, -1, 0, // LR
        -1, 1, 0, // UL
    ];

    gl.bufferData(gl.ARRAY_BUFFER, new Float32Array(triangleVertices), gl.STATIC_DRAW);

    buffer.itemSize = 3;
    buffer.numberOfItems = triangleVertices.length / 3;
}

function cleanup() {
    gl.useProgram(null);
    if (buffer) {
        gl.deleteBuffer(buffer);
    }
    if (program) {
        gl.deleteProgram(program);
    }
}

function getRenderingContext() {
    text = document.querySelector('#text');

    const canvas = document.querySelector('#canvas');
    
    canvas.width = canvas.clientWidth;
    canvas.height = canvas.clientHeight;
    canvas.onmousemove = mousemove;

    const gl = canvas.getContext("webgl") || canvas.getContext("experimental-webgl");
    if (!gl) {
        const paragraph = document.querySelector("p");
        paragraph.textContent = "Failed. Your browser or device may not support WebGL.";
        return null;
    }

    //gl.viewport(0, 0, gl.drawingBufferWidth, gl.drawingBufferHeight);
    gl.clearColor(0.0, 0.0, 0.0, 1.0);
    gl.clear(gl.COLOR_BUFFER_BIT);

    gl.viewportWidth = canvas.clientWidth;
    gl.viewportHeight = canvas.clientHeight;

    return gl;
}


