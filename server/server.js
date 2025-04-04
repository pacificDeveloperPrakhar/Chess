const mongoose = require("mongoose");
const dotenv = require("dotenv");
// this is our coded engine module natively coded in the rust and then built a binary file for the
// current platform
const {allPossibleMoves,isValidMove}=require("engine_lib");
dotenv.config({ path: "./config.env" });
const cors = require("cors");
const shiftChar=require("./utils/shiftChar.js")
const http = require("http");
const geturl = require("./utils/get_connection_url.js");
const app = require("./app.js");
const server = http.createServer(app);
const host = process.env.host;
const port = process.env.port;
const {Server:socket}=require("socket.io")
const {colorBright,colorFgCyan,colorRed,colorReset,colorPurple, colorFgGreen}=require("./color_codes.js");
const { format } = require("path");

console.log(`${colorFgCyan}mode:${colorReset} ${colorBright}${colorPurple}${process.env.mode}${colorReset}`);
// if there is an unhadled promise then we have an even registered for it here
process.on("unhandledRejection", (err) => {
  console.log(err);
  server.close(() => {
    process.exit();
  });
});
// if there is an unacaught exception,then we have an event registered for it
process.on("uncaughtException", (err) => {
  console.log(err);
  server.close(() => {
    process.exit();
  });
});

const url = geturl("test", process.env.user, process.env.password);

mongoose.connect(url).catch((err) => console.log("error encoutered"));
const db = mongoose.connection;
db.once("open", () => {
  const portused = db.port;
  console.log("\033[104mPort used for mongoDB connection:\033[0m",portused);

});
db.on("error", (err) => {
  console.log(err);
});

server.listen(port, host, (err) => {
  if (err) {
    console.error(`Error starting the server: ${err}`);
  } else {
    console.log(
      `${colorBright}${colorFgGreen}Server is listening on ${colorFgCyan}http://${host}:${port}${colorReset}`
    );
  }
});
const io=new socket(server,{
  path:"/interface_chess/"
})
io.engine.use(cors({
  origin: "http://localhost:3000", // The frontend origin
  methods: ["GET", "POST"], // Allowed HTTP methods
  credentials: true // Allow cookies to be sent with the request (if needed)
}))
io.on("connection",async(socket)=>{
  
  console.log("connected to the server");
  socket.on("disconnect",()=>{
    console.log("disconnected");
  })
  socket.on("initialize",async(id)=>{
    const current_connections=(await io.in(id).fetchSockets()).length;
    if (!current_connections){
      memory_base[id]=null
        
    }
    
    socket.join(id);
    console.log(socket.rooms);
    console.log(`joined the room ${id}`);
    console.log(current_connections)
    io.to(id).emit("joined",{id,current_connections});

  })
  socket.on("piece_selected",async (data)=>{
    console.log(socket.rooms);
    await state_select(memory_base,io,socket,data)
  })
  socket.on("move_selected",(data)=>{
    console.log("move was selected");
  })
})

const memory_base={}
async function state_select(memory_base,io,socket,data){
  const id=Array.from(socket.rooms)[1];

  // if(!memory_base.id&&data.color!='a')
  // {return}
  // if(memory_base.id&&data.color!=memory_base.id.color){
  //   return
  // }
  if(data.type!=='moveType'){
    memory_base.id=data
    if(data.color=='a')
      { 
        data.epd=data.epd.split('/').reverse().join('/');
      }
    else if(data.color=='A'){
      data.move[0]=7-data.move[0];
    }
      data.move[0]=7-data.move[0]
      console.log(data.epd);
      data.move=shiftChar(data.move[0],'a')+shiftChar(data.move[1],'1')
      console.log(data.move);
      let all_moves=allPossibleMoves(data.epd,data.move);
    console.log(all_moves);
    io.to(socket.id).emit("moves_state_change",all_moves);
    return
  }

    // this will executed when the type is move i will check if it is valid if valid then take the epd from
    // the staak and update it with the position of the move and emit it to all socket 
  //  also at the end empty the stack

}
console.log(shiftChar(1,'a'))