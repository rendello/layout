import init, { tokenize } from './inuklib.js';

async function run() {
    await init();
    window.tokenize = tokenize;
    document.querySelectorAll('p').forEach(element => {
        if (element.innerText !== null) {  
            element.innerHTML = tokenize(element.innerText);
        }
    });
}

run();