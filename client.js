// const net = require('net');
const cluster = require('cluster');


let message = Buffer.from(`Rc8rghWRhEA9cVzPasoF4zmPA8xff2yphui2o5YQJUlk4H8VQK7pZkrYorNsrIiORc8rghWRhEA9cVzPasoF4zmPA8xff
2yphui2o5YQJUlk4H8VQK7pZkrYorNsrIiORc8rghWRhEA9cVzPasoF4zmPA8xff2yphui2o5YQJUlk4H8VQK7pZkrYorNsrIiORc8rghWRhEA9cVzP
asoF4zmPA8xff2yphui2o5YQJUlk4H8VQK7pZkrYorNsrIiORc8rghWRhEA9cVzPasoF4zmPA8xff2yphui2o5YQJUlk4H8VQK7pZkrYorNsrIiO
Rc8rghWRhEA9cVzPasoF4zmPA8xff2yphui2o5YQJUlk4H8VQK7pZkrYorNsrIiORc8rghWRhEA9cVzPasoF4zmPA8xff
2yphui2o5YQJUlk4H8VQK7pZkrYorNsrIiORc8rghWRhEA9cVzPasoF4zmPA8xff2yphui2o5YQJUlk4H8VQK7pZkrYorNsrIiORc8rghWRhEA9cVzP
asoF4zmPA8xff2yphui2o5YQJUlk4H8VQK7pZkrYorNsrIiORc8rghWRhEA9cVzPasoF4zmPA8xff2yphui2o5YQJUlk4H8VQK7pZkrYorNsrIiO
Rc8rghWRhEA9cVzPasoF4zmPA8xff2yphui2o5YQJUlk4H8VQK7pZkrYorNsrIiORc8rghWRhEA9cVzPasoF4zmPA8xff
2yphui2o5YQJUlk4H8VQK7pZkrYorNsrIiORc8rghWRhEA9cVzPasoF4zmPA8xff2yphui2o5YQJUlk4H8VQK7pZkrYorNsrIiORc8rghWRhEA9cVzP
asoF4zmPA8xff2yphui2o5YQJUlk4H8VQK7pZkrYorNsrIiORc8rghWRhEA9cVzPasoF4zmPA8xff2yphui2o5YQJUlk4H8VQK7pZkrYorNsrIiO
Rc8rghWRhEA9cVzPasoF4zmPA8xff2yphui2o5YQJUlk4H8VQK7pZkrYorNsrIiORc8rghWRhEA9cVzPasoF4zmPA8xff
2yphui2o5YQJUlk4H8VQK7pZkrYorNsrIiORc8rghWRhEA9cVzPasoF4zmPA8xff2yphui2o5YQJUlk4H8VQK7pZkrYorNsrIiORc8rghWRhEA9cVzP
asoF4zmPA8xff2yphui2o5YQJUlk4H8VQK7pZkrYorNsrIiORc8rghWRhEA9cVzPasoF4zmPA8xff2yphui2o5YQJUlk4H8VQK7pZkrYorNsrIiO
Rc8rghWRhEA9cVzPasoF4zmPA8xff2yphui2o5YQJUlk4H8VQK7pZkrYorNsrIiORc8rghWRhEA9cVzPasoF4zmPA8xff
2yphui2o5YQJUlk4H8VQK7pZkrYorNsrIiORc8rghWRhEA9cVzPasoF4zmPA8xff2yphui2o5YQJUlk4H8VQK7pZkrYorNsrIiORc8rghWRhEA9cVzP
asoF4zmPA8xff2yphui2o5YQJUlk4H8VQK7pZkrYorNsrIiORc8rghWRhEA9cVzPasoF4zmPA8xff2yphui2o5YQJUlk4H8VQK7pZkrYorNsrIiO
Rc8rghWRhEA9cVzPasoF4zmPA8xff2yphui2o5YQJUlk4H8VQK7pZkrYorNsrIiORc8rghWRhEA9cVzPasoF4zmPA8xff
2yphui2o5YQJUlk4H8VQK7pZkrYorNsrIiORc8rghWRhEA9cVzPasoF4zmPA8xff2yphui2o5YQJUlk4H8VQK7pZkrYorNsrIiORc8rghWRhEA9cVzP
asoF4zmPA8xff2yphui2o5YQJUlk4H8VQK7pZkrYorNsrIiORc8rghWRhEA9cVzPasoF4zmPA8xff2yphui2o5YQJUlk4H8VQK7pZkrYorNsrIiO:`);

if (cluster.isMaster) {
    for (let i = 0; i < 10; i++) {
        cluster.fork();
    }
    return
}

// console.log(message.length)
// let socket = net.connect(3000, '127.0.0.1');

// socket.on('connect', () => {
//     console.log("connected ")
//     socket.write(message);
// })

// socket.on('data', () => {
//     socket.write(message)
// })

// setTimeout(() => {
//     socket.end();
// }, 10000);

// const uws = require('clusterws-uws');

// new uws.WebSocketServer({ port: 3000 }, () => {
//     console.log('Server is running');
// });

// console.log(message.length)

const net = require('net');

let ended = false;

let socket = net.connect(3000, '127.0.0.1');
// socket.write('Hello from js');


socket.on('connect', () => {
    console.log("open");
    socket.write(message)

    // setTimeout(() => {
    //     socket.write("Second Hello world")

    // }, 5000);
    //     console.log('Socket connected');
});
let num_msg = 0;
socket.on('data', (data) => {
    num_msg++;
    // if (!ended) {
    if (socket.writable) {
        socket.write(message)
    }
    // setTimeout(() => {

    // }, 5000)
    // }
    // console.log("I am hrere:", data.toString() + '\n');
});

socket.on('error', () => { })

setTimeout(() => {
    console.log(num_msg);
    socket.end();
    // ended = true
    // process.exit();
}, 10000)
