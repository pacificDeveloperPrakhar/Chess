"use client"
import "@/style/chess_board.css"
import clsx from "clsx";
import { useState } from "react"
import Image from "next/image";
import {motion,useDragControls} from "motion/react";
export default function Board(){
    // the game initial overall board piece do be rendered accordingly
    const [state,setState]=useState(Array.from({length:8},()=>Array(8).fill(0)));
    // this is the epd notation which do be used to render the state of the chess game
    const [epd,setEpd]=useState("rnbqkbnr/pppppppp/00000000/0000Q000/00P0P000/00000000/PPPp0PPp/RNBQKBNR".split("/").reverse().map((row)=>row.split("")));
    const controls=useDragControls();
    return<> 
        <div className="board  absolute "> 
            {state.map((squares,row)=>{

                return squares.map((square,col)=>{
                const isUpperCase=/^(?=[A-Z])[A-Z\s]+$/;
                const piece_to_be_placed=isUpperCase.test(epd[row][col])?epd[row][col]+"b":epd[row][col];
                console.log(piece_to_be_placed.toLowerCase());

                return <>
                <div className={clsx(`${(col+row)%2==0?"square_white":"square_black"}`, {})
                }>
                    <motion.div className="square_piece" drag  dragControls={controls}>
                        { 
                          
                          (epd[row][col] !== "0" && epd[row][col] !== "-" && epd[row][col] !== ".") ? (
                            <Image
                              src={`/chess_piece/${piece_to_be_placed.toLowerCase()}.svg`}
                              width={59}
                              height={59}
                              alt="rook"
                            />
                          ) : (

                            epd[row][col] === "-" ? (
                                <div className="bg-red-400/60 w-full h-full backdrop-blur-3xl"></div>
                            ) : epd[row][col] === "." ? (
                                <div className="bg-green-600/60 w-full h-full backdrop-blur-3xl"></div>
                            ) :
                            <></>
                          )
                          
                        }
                    </motion.div>
                </div>
                </>})
            })}
        </div>
    </>
}