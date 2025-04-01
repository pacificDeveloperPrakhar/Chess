"use client"
import "@/style/chess_board.css"
import clsx from "clsx";
import { useEffect, useState } from "react"
import Image from "next/image";
import { useParams } from "next/navigation";
import {motion,useDragControls} from "motion/react";
import { io } from "socket.io-client";
import { h1 } from "motion/react-client";
export default function Board(){
    // the game initial overall board piece do be rendered accordingly
    const [state,setState]=useState(Array.from({length:8},()=>Array(8).fill(0)));
    const [room,setRoom]=useState({id:null,current_connections:0});
    const { id } = useParams();
    console.log("this is the room",id);
    const socket = io("http://127.0.0.1:1234", {
        path: "/interface_chess/"
      });
    // this is the epd notation which do be used to render the state of the chess game
    const [epd,setEpd]=useState("rnbqkbnr/pppppppp/00000000/000p0000/00P0P000/00000000/00P00000/rNQqKBNR".split("/").reverse().map((row)=>row.split("")));
    const [epd_display,setEpdDisplay]=useState("00000000/00000-0-/000000../000-..../000000../00000.0./0000.00./000-0000".split("/").reverse().map((row)=>row.split("")));
    useEffect(()=>{
      socket.emit("initialize",id);
      socket.on("joined",async (data)=>{
        const {id,current_connections}=data;
        console.log(current_connections);
        setRoom({
          id,current_connections
        });
      })
      
    },[])
    return<> 
        <div className="board  absolute "> 
            {state.map((squares,row)=>{

                return squares.map((square,col)=>{
                const isUpperCase=/^(?=[A-Z])[A-Z\s]+$/;
                const piece_to_be_placed=isUpperCase.test(epd[row][col])?epd[row][col]+"b":epd[row][col];
                return <>
                <div className={clsx(`${(col+row)%2==0?"square_white":"square_black"}`, {})
                }>
                    <motion.div className={clsx("square_piece",{
            'kill_move': epd_display[row][col] === '-',
            'movable_move': epd_display[row][col] === '.'
        })} >
                        
                          
                          {(epd[row][col] !== "0" && epd[row][col] !== "-" && epd[row][col] !== ".") ? (
                            <Image
                              src={`/chess_piece/${piece_to_be_placed.toLowerCase()}.svg`}
                              width={59}
                              height={59}
                              alt={piece_to_be_placed}
                            />
                          ) :<></>}
                          
                        
                    </motion.div>
                </div>
                </>})
            })}
        </div>
        <div className=" absolute right-0">
          {room&&<h1>room joined:{room.id} </h1>}
          {room.current_connections&&<h1>connected sockets:{room.current_connections}</h1>}
        </div>
    </>
}