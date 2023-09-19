// This javascript file is included in the index.html file
// It is the main javascript file for the application
// It periodically pulls data from the server and updates the display

// includes dynamictable.js, which contains the code to create the table
import dynamic from "./dynamic.js";

var dyn = new dynamic();

// on page load without using jQuery, do this
window.onload = function() {
  // set the interval to 5 seconds
  setInterval(function() {
    // call the function to update the display
    dyn.updateDisplay();
  }, 5000);
};