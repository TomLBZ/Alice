// this function gets nodeID from the backend
function getNodeID() {
    // create a variable to hold the nodeID
    var nodeID;
    // create a variable to hold the url
    var url = "http://localhost:8080/api/open/nodeID";
    // the backend api returns nodeID as plain text
    // use jQuery to get the data from the server
    $.get(url, function(data) {
        // set the nodeID variable to the data
        nodeID = data;
    });
    // return the nodeID
    return nodeID;
}

// this function is called when the page loads
function updateDisplay() {
  // get the data from the server
  var data = getJSONData();
  // create the table
  createTable(data);
}

// this function gets the data from the server
function getJSONData() {
    // create a variable to hold the data
    var data;
    // create a variable to hold the url
    var url = "http://localhost:8080/api/open/json";
    // use jQuery to get the data from the server
    $.getJSON(url, function(json) {
        // set the data variable to the json data
        data = json;
    });
    // return the data
    return data;
    }

// this function creates the table
function createTable(data) {
    // create a variable to hold the table html
    var table = "<table>";
    // loop through the data
    for (var i = 0; i < data.length; i++) {
        // create a variable to hold the row html
        var row = "<tr>";
        // add the data to the row
        row += "<td>" + data[i].id + "</td>";
        row += "<td>" + data[i].name + "</td>";
        row += "<td>" + data[i].description + "</td>";
        row += "<td>" + data[i].price + "</td>";
        row += "<td>" + data[i].quantity + "</td>";
        // close the row
        row += "</tr>";
        // add the row to the table
        table += row;
    }
    // close the table
    table += "</table>";
    // set the html of the table element to the table
    $("#table").html(table);
    }