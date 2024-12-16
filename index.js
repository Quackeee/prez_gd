// prevents context (right-click) menu from appearing.
const canvas = document.getElementById("canvas");
canvas.oncontextmenu = (e) => e.preventDefault();

// focus the canvas to enable keyboard input
canvas.focus();

import('./pkg')
  .catch(console.error);
