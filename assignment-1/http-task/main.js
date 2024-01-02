import { get } from 'http';
// const http=require('http')
// async function getData(){
//     let res=await http.get(" ")
//     let data=res.json()
//     console.log(data);
// }
// var xhr = new XMLHttpRequest();
// xhr.open('GET', 'http://localhost:3000/users', true);
// xhr.setProtocol('HTTP/1.0'); // Set the HTTP protocol explicitly

// xhr.onreadystatechange = function() {
//   if (xhr.readyState == 4 && xhr.status == 200) {
//     var responseData = JSON.parse(xhr.responseText);
//     console.log(responseData);
//   }
// };

// xhr.send();

// getData()

// const https = require('https');

get('http://localhost:3000/users', res => {
  let data = [];

  res.on('data', chunk => {
    data.push(chunk);
  });

  res.on('end', () => {
    console.log('Response ended: ');
    console.log(data);

    // for(user of users) {

    //   console.log(`Got user with id: ${user.id}, name: ${user.name}`);
    // }
  });
}).on('error', err => {
  console.log('Error: ', err.message);
});
