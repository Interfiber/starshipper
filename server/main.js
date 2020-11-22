const express = require('express')
const app = express()
const port = 5656
app.get('/create_account', (req, res) => {
  console.clear();
  console.log("Client as connected!");
  console.log("Fetching headers...");
  const username = req.headers.username;
  const password = req.headers.password;
  console.log("Fetched headers.");
  console.log("Creating account for " + username + "...");
  console.log("Creating account object...");
  const account = {
    username: username,
    password: password
  };
  var mongo = require('mongodb')
  var MongoClient = require('mongodb').MongoClient;
  var url = "mongodb://localhost:27017/starshipper";
  MongoClient.connect(url,
    function(err, db) {
  if (err) throw err;
    console.log("Database connected!");
    var dbo = db.db("users");
    dbo.collection("users").insertOne(account, function(err, res) {
        if (err) throw err;
        console.log("1 document inserted");
        var results = dbo.collection("users").find({});
        console.log("Added user! List of users:");
        results.forEach(row => {
           const rowJson = JSON.parse(JSON.stringify(row));
           console.log(rowJson.username);
       });
       db.close();
     });
  });

  res.send('Account Created')
})
app.get("/save", (req, res) => {
  console.clear();
  console.log("Client has connected!");
  console.log("Parsing headers...");
  const outputFile = req.headers.filename;
  const line = req.headers.line;
  console.log("Checking for file " + outputFile + "...");
  let fs = require("fs");
  if (!fs.existsSync(outputFile)){
    console.log("File does not exist creating...");
    fs.writeFile(outputFile, "", (err) => {
    if (err) throw err;
      console.log("File Created.");
    });
  }
  console.log("Appending file data...");
  fs.appendFile(outputFile, line + "\n", (err) => {
    if (err) {
        throw err;
    }
    console.log("Added line.");
  });
  console.log("Sending 'Done' to Client...");
  res.send('done');
  console.log("Done.");
});
app.get("/download", (req, res) => {
  console.clear();
  console.log("Client connected!");
  console.log("Parsing headers...");
  let uuid = req.headers.uuid;
  console.log("Testing if file exists...");
  const fs = require("fs");
  let file_data = fs.readFileSync(uuid)
  res.send(file_data)
})
app.get("/delete", (req, res) => {
  console.clear();
  console.log("Client connected!");
  console.log("Parsing Headers...");
  let uuid = req.headers.uuid;
  console.log("Deleting file...");
  const fs = require("fs");
  fs.unlinkSync(uuid);
  res.send("OK")
})
app.listen(port, () => {
  console.log(`Backend is listening on http://localhost:${port}`)
})
