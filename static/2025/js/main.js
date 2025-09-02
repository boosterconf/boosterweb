function shuffle(parent) {
    if(parent != null) {
        for (var i = parent.children.length; i >= 0; i--) {
            parent.appendChild(parent.children[Math.random() * i | 0]);
        }
    }
}
var sponsorlogos = document.querySelector('.sponsorlogos');
shuffle(sponsorlogos);
var organizers = document.querySelector('#organizers');
shuffle(organizers);


// I can't believe it's not a touch screen
document.addEventListener('DOMContentLoaded', function () {
    const elems = document.getElementsByClassName('grabbable');
    for (let i = 0; i < elems.length; i++) {
        let ele = elems[i];
        ele.style.cursor = 'grab';

        let pos = { top: 0, left: 0, x: 0, y: 0 };
    
        const mouseDownHandler = function (e) {
            ele.style.cursor = 'grabbing';
            ele.style.userSelect = 'none';
    
            pos = {
                left: ele.scrollLeft,
                top: ele.scrollTop,
                // Get the current mouse position
                x: e.clientX,
                y: e.clientY,
            };
    
            document.addEventListener('mousemove', mouseMoveHandler);
            document.addEventListener('mouseup', mouseUpHandler);
        };
    
        const mouseMoveHandler = function (e) {
            // How far the mouse has been moved
            const dx = e.clientX - pos.x;
            const dy = e.clientY - pos.y;
    
            // Scroll the element
            ele.scrollTop = pos.top - dy;
            ele.scrollLeft = pos.left - dx;
        };
    
        const mouseUpHandler = function () {
            ele.style.cursor = 'grab';
            ele.style.removeProperty('user-select');
    
            document.removeEventListener('mousemove', mouseMoveHandler);
            document.removeEventListener('mouseup', mouseUpHandler);
        };
    
        // Attach the handler
        ele.addEventListener('mousedown', mouseDownHandler);
    }
});
