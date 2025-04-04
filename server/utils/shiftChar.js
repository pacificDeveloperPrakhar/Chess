module.exports=function shiftChar(n, char){
    if (char.length !== 1) throw new Error("Input must be a single character.");
    return String.fromCharCode(char.charCodeAt(0) + n);
}