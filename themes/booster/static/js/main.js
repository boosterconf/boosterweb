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
