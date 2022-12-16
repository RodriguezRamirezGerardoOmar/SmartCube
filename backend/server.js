const http = require('http');
const url = require('url');
const express = require('express');
const bodyParser = require('body-parser');
const fs = require('fs');

const hostname = '127.0.0.1';
const port = 5000;

const server = express();

server.use(bodyParser.urlencoded({
	extended:true
}));

server.post("/", function(req, res){
	res.send(JSON.stringify(req.body));
});

server.listen(port, () => {
	console.log(hostname.toString() + ' ' + port.toString());
});