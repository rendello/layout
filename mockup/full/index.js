
document.addEventListener('click', function(event) {
    if (event.ctrlKey) {
        if (event.target.style.background === 'none') {
            event.target.style.background = '';
        } else {
            event.target.style.background = 'none';
        }
    }
});
