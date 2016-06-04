function json_serialize_sudoku() {
    var root = document.getElementById("sudoku-grid");
    var cells = document.getElementsByClassName("sudoku-cell__input");
    var rows = new Array(9).fill("");
    for (var i=0; i<cells.length; i++) {
        var index = 3*Math.floor(i/27) + (Math.floor(i/3) % 3);
        if (cells[i].value.trim() != "") {
            rows[index] += cells[i].value + ',';
        }
        else {
            rows[index] += "_" + ',';
        }
    }

    for (var i=0; i<rows.length; i++){
        rows[i] = rows[i].substring(0, rows[i].length-1);
    }

    return JSON.stringify(rows);
}

function unserialize_sudoku(s) {
    var root = document.getElementById("sudoku-grid");
    var cells = document.getElementsByClassName("sudoku-cell__input");
    var rows = s.trim().split('\n');
    for (var i=0; i<rows.length; i++) {
        var numbers = rows[i].split(',');
        for (var j=0; j<numbers.length; j++) {
            var index = 27*Math.floor(i/3) + 3*(i % 3) + 9*Math.floor(j/3) + j % 3;
            console.log(index);
            if (numbers[j] == "_") {
                cells[index].value = "";
            }
            else {
                cells[index].value = numbers[j];
            }
        }
    }
}

function create_sudoku_block() {
    var root = document.createElement("table");
    root.classList.add("sudoku-block");
    for (var i=0; i<3; i++) {
        var row = document.createElement("tr");
        for (var j=0; j<3; j++) {
            var column = document.createElement("td");
            column.classList.add("sudoku-cell");
            var input = document.createElement("input");
            input.classList.add("sudoku-cell__input");
            input.type = "text";
            column.appendChild(input);
            row.appendChild(column);
        }
        root.appendChild(row);
    }
    return root;
}

function create_sudoku_grid() {
    var root = document.getElementById("sudoku-grid");
    for (var i=0; i<3; i++) {
        var blockrow = document.createElement("div");
        for (var j=0; j<3; j++) {
            var block = document.createElement("span");
            block.appendChild(create_sudoku_block());
            blockrow.appendChild(block);
        }
        root.appendChild(blockrow);
    }
}

function send_msg() {
    document.getElementById("message-box").innerHTML = "loading...";
//    var msg = document.getElementById("msg-input").value;
    var msg = json_serialize_sudoku();
    socket.send(msg);
}

function onmessage(event) {
    if (event.data.length < 20) {
        document.getElementById("message-box").innerHTML = event.data;
    }
    else {
        unserialize_sudoku(event.data);
        document.getElementById("message-box").innerHTML = "Solved!";
    }
}

function onopen(event) {
    document.getElementById("message-box").innerHTML = "Connected successfully!";
    document.getElementById("connect-button").classList.add("hidden");
}

function onclose(event) {
    document.getElementById("connect-button").classList.remove("hidden");
}

var socket;

function connect() {
    socket = new WebSocket("ws://localhost:3012");
    socket.onmessage = onmessage;
    socket.onopen = onopen;
    socket.onclose = onclose;
}
