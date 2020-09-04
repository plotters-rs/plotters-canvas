/*!
   The Plotters HTML5 Canvas backend.

   This backend allows Plotters operates the HTML5 Canvas when targeted to Webassembly.

   See the documentation for [CanvasBackend](struct.CanvasBackend.html) for more details.
*/
mod canvas;

pub use canvas::CanvasBackend;
