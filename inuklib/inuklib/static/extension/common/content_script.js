function injectScript(file, node) {
    var th = document.getElementsByTagName(node)[0];
    var s = document.createElement('script');
    s.setAttribute('type', 'module');
    s.setAttribute('src', chrome.runtime.getURL(file));
    th.appendChild(s);
    var style = document.createElement('link');
    style.setAttribute('rel', 'stylesheet');
    style.setAttribute('href', chrome.runtime.getURL('inuktitut_style.css'))
    th.appendChild(style);
}
injectScript('init.js', 'body');
