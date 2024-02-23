const fs = require('fs');
const {table_hmap} = require("./pkg/table_task_rust.js");
const { log } = require('console');
// let table_data;
(function make_table(){
    fs.readFile('./d.json', 'utf8', (err, data)=>{
    if(err){
        console.log(err);
        return;
    }
    try{
        let result=table_hmap(JSON.stringify(JSON.parse(data)));
        console.log(result);
    }
    catch(err){
        console.log(err);
    }
})
    
    // console.log(JSON.stringify(table_data));
    // console.log(table_hmap(JSON.stringify(data)))
})();
// global.get_table_data=()=> {
    
//     return JSON.stringify(table_data);
// }

