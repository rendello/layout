import init, { tokenize } from './analyzer.js';

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