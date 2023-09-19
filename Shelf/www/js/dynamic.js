export default class dynamic {
    // public variables
    nodeID = "";

    constructor() {
        var url = "http://127.0.0.1:8000/api/open/nodeID";
        // get plain text from the server without using jQuery
        var request = new XMLHttpRequest();
        request.onreadystatechange = function() {
            if (request.readyState == 4 && request.status == 200) {
                console.log("TESTLOG: " + request.responseText);
                var spanNodeID = document.getElementById("nodeID");
                spanNodeID.innerHTML = request.responseText;
            }
        }
        request.open("GET", url, false);
        request.send();
    }

    // this function is called when the page loads
    updateDisplay() {
        // get the data from the server
        var data = this.getJSONData();
        // create the table
        this.createTable(data);
    }

    // this function gets the data from the server
    getJSONData() {
        // create a variable to hold the data
        var data;
        // create a variable to hold the url
        var url = "http://127.0.0.1:8000/api/open/json";
        // get json from server without using jquery
        var request = new XMLHttpRequest();
        request.onreadystatechange = function() {
            if (request.readyState == 4 && request.status == 200) {
                // parse the json
                data = JSON.parse(request.responseText);
            }
        }
        request.open("GET", url, false);
        request.send();
        // return the data
        return data;
    }

    // this function creates the table
    createTable(data) {
        var tablebody = document.getElementById("dbdata");
        // checks if the data is null or empty
        if (data == null || data.length == 0) {
            tablebody.innerHTML = "No data found";
            return;
        }
        tablebody.innerHTML = "";
        // for each table entry in the data
        for (var i = 0; i < data.length; i++) {
            var state = data[i].state;
            var id = data[i].id;
            var name = data[i].name;
            var description = data[i].description;
            var instances = data[i].instances;
            var calls = data[i].calls;
            var errors = data[i].errors;
            var latency = data[i].latency;
            var ram = data[i].ram;
            var uptime = data[i].uptime;
            var endpoint = data[i].endpoint;
            // create a multi-line string as a row in the table, with variables
            var rowText = 
            `<tr>
                <td class="icon"><svg>
                <circle cx="10" cy="10" r="9" class="${state ? "running" : "stopped" }" />
                </svg></td>
                <td>${id}</td>
                <td class="tooltip">${name}
                <span class="tooltiptext">${description}</span></td>
                <td>${instances}</td>
                <td>${calls}</td>
                <td>${errors}</td>
                <td>${latency}</td>
                <td>${ram}</td>
                <td>${uptime}</td>
                <td><a href="${endpoint}">${name}</a></td>
            <tr>`;
            // add the row to the table
            tablebody.innerHTML += rowText;
        }
    }    
}