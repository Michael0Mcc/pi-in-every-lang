// const PI = 3.141592653589793;

let pi = 4.0;
let div = 1.0;
for (let i = 0; i < Number.MAX_VALUE; i++) {
    div += 2.0;
    if (i % 2 === 0) {
        pi -= 4.0/div;
    } else {
        pi += 4.0/div;
    }
    console.log(`approx: ${pi},\t % err: ${100 * Math.abs(pi - Math.PI) / Math.PI}\n`);
}