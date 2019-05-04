const express = require('express');
const bodyParser = require('body-parser');
const path = require('path');
const os = require('os');
const fs = require('fs');
const shell = require('shelljs');
 
const app = express();

app.use('/stuff',express.static(__dirname + '/stuff'));
app.use('/ace',express.static(__dirname + '/ace'));

app.use(bodyParser.urlencoded({extended : true}));
app.use(bodyParser.json());

app.get('/',function(request,response){
    response.sendFile(path.join(__dirname,'html/editor.html'));
});


app.post('/saveFile',function(request,response){
    var data = request.body.text;
    console.log(data);
    fs.writeFile('thisCode.tiny',data,(err)=>{
        if(err) throw err;
        runTehParsah();
        response.redirect('/tree');
    });
});

app.get('/tree',function(request,response){
    response.sendFile(path.join(__dirname, 'html/tree.html')); 
});

app.get('/tree/get',function(request,response){
    fs.readFile('./json.json', (err, data) => {  
        if (err) throw err;
        let student = JSON.parse(data);
        console.log( student);
        response.send(student);
    });
});

function runTehParsah(){

    if(os.platform=="win32"){
        shell.exec('tiny_parser.exe thisCode.tiny json.json true')

    }
}
console.log("Listening");
app.listen(3000  , '0.0.0.0');
