// This javascript file is included in the index.html file
// It is the main javascript file for the application
// It periodically pulls data from the server and updates the display

// includes dynamictable.js, which contains the code to create the table
include("dynamictable.js");


// on page load, do this
$(document).ready(function() {
    // get nodeID from file
    var nodeID = getNodeID();
    // set the nodeID in the html
    $("#nodeID").html(nodeID);
  // set the interval to 5 seconds
  setInterval(function() {
    // call the function to update the display
    updateDisplay();
  }, 5000);
});