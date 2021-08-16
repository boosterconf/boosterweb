$( document ).ready(function($) {
    function shuffle(o) {
        for(var j, x, i = o.length; i; j = Math.floor(Math.random() * i), x = o[--i], o[i] = o[j], o[j] = x);
        return o;
    };
    $(".sponsorlogos").html(shuffle($(".sponsorlogos li")));

    $("#organizers").html(shuffle($("#organizers li")));
});