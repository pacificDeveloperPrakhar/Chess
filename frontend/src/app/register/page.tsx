"use client";
import { useRouter } from "next/navigation";
import { useState } from "react";

export default function Registration() {
    const router = useRouter();
    const [id, setId] = useState("");

    const handleNavigate = () => {
        if (!id) return; 
        router.push(`/board/${id}`); 
    };

    return (
        <>
            <input 
                type="text" 
                id="socket_id" 
                onChange={(e) => setId(e.target.value)} 
                value={id} 
                placeholder="Enter Board ID"
            />
            <button onClick={handleNavigate}>Play</button>
        </>
    );
}
