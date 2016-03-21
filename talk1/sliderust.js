window.addEventListener('load', function () {
    // Put each <h1> and subsequent content into its own <section>.
    function makeSections(node) {
        var sections = [];
        while (true) {
            sections.push(document.createElement('section'));
            var section = sections[sections.length - 1];
            var sib;
            while (true) {
                sib = node.nextSibling;
                if (sib === null) {
                    section.appendChild(node);
                    return sections;
                }
                if (sib.nodeName === 'H1') {
                    break;
                }
                section.appendChild(node);
                node = sib;
            }
            node = sib;
        }
    }

    var title = document.getElementsByClassName('title')[0];
    [].forEach.call(makeSections(title), function (x) {
        document.body.appendChild(x);
    });

    var sections = document.getElementsByTagName('section');
    var current = 0;
    function update() {
        [].forEach.call(sections, function (x, i) {
            x.className = (i === 0) ? 'title-slide' : '';
        });
        if (current < 0)
            current = 0;
        if (current >= sections.length)
            current = sections.length - 1;
        sections[current].className += ' current';
    }

    update();

    document.body.addEventListener('keydown', function (ev) {
        switch (ev.keyCode) {
            case 39: current++; break;
            case 37: current--; break;
        }
        update();
    });
});

// countdown from http://codepen.io/SitePoint/pen/MwNPVq
// MIT licensed, by SitePoint
function getTimeRemaining(endtime) {
  var t = Date.parse(endtime) - Date.parse(new Date());
  var seconds = Math.floor((t / 1000) % 60);
  var minutes = Math.floor((t / 1000 / 60) % 60);
  var hours = Math.floor((t / (1000 * 60 * 60)) % 24);
  var days = Math.floor(t / (1000 * 60 * 60 * 24));
  return {
    'total': t,
    'days': days,
    'hours': hours,
    'minutes': minutes,
    'seconds': seconds
  };
}

function initializeClock(id, endtime) {
  var clock = document.getElementById(id);
  var daysSpan = clock.querySelector('.days');
  var hoursSpan = clock.querySelector('.hours');
  var minutesSpan = clock.querySelector('.minutes');
  var secondsSpan = clock.querySelector('.seconds');

  function updateClock() {
    var t = getTimeRemaining(endtime);

    clock.innerHTML = (t.days + " days "  + ('0' + t.hours).slice(-2) + "h " + ('0' + t.minutes).slice(-2) + "m " + ('0' + t.seconds).slice(-2)) + "s";

    if (t.total <= 0) {
      clearInterval(timeinterval);
    }
  }

  updateClock();
  var timeinterval = setInterval(updateClock, 1000);
}

window.addEventListener('load', function () {
	var deadline = new Date(Date.parse("2016-04-13"));
	initializeClock('stableclock', deadline);
});
