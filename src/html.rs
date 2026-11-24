//! Single-page HTML document. One static string: zero per-request allocation.

pub const PAGE: &str = r##"<!DOCTYPE html>
<html lang="en" data-type="place">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<meta name="page-type" content="place">
<meta name="site-kind" content="other">
<meta name="page-kind" content="place">
<meta name="description" content="モネ — a Pom Poodle mix in Yokohama. Eats her own poop. Barks at everyone.">
<meta name="theme-color" content="#FFF9F6">
<title>モネ — Pom Poodle in Yokohama</title>
<link rel="icon" href="data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 100 100'%3E%3Ccircle cx='50' cy='50' r='48' fill='%23E8756A'/%3E%3Ccircle cx='34' cy='42' r='6' fill='%233D2B26'/%3E%3Ccircle cx='66' cy='42' r='6' fill='%233D2B26'/%3E%3Cellipse cx='50' cy='62' rx='14' ry='10' fill='%23FFE3DF'/%3E%3Ccircle cx='50' cy='55' r='5' fill='%233D2B26'/%3E%3Cpath d='M38 62 Q50 72 62 62' stroke='%233D2B26' stroke-width='3' fill='none' stroke-linecap='round'/%3E%3C/svg%3E">
<link rel="preconnect" href="https://fonts.googleapis.com">
<link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
<link href="https://fonts.googleapis.com/css2?family=Nunito:wght@400;600;700;800;900&family=Varela+Round&display=swap" rel="stylesheet" media="print" onload="this.media='all'">
<noscript><link href="https://fonts.googleapis.com/css2?family=Nunito:wght@400;600;700;800;900&family=Varela+Round&display=swap" rel="stylesheet"></noscript>
<style>
:root{
  --bg:#FFF9F6;
  --surface:#FFFFFF;
  --ink:#3D2B26;
  --ink-soft:#6B4F47;
  --accent:#E8756A;
  --accent-deep:#D45A50;
  --line:#F0DDD8;
  --radius:20px;
  --spring:cubic-bezier(.32,.72,0,1);
  --ease:cubic-bezier(.25,.46,.45,.94);
}
*,*::before,*::after{margin:0;padding:0;box-sizing:border-box}
html{scroll-behavior:smooth}
body{
  font-family:"Nunito",system-ui,sans-serif;
  font-size:16px;line-height:1.65;
  color:var(--ink);
  background:var(--bg);
  min-height:100dvh;
  overflow-x:hidden;
  -webkit-font-smoothing:antialiased;
}
img{display:block;max-width:100%;height:auto}
a{color:var(--accent-deep);text-decoration:none}
h1,h2,h3{font-family:"Varela Round","Nunito",sans-serif;letter-spacing:-.025em;line-height:1.1}
h1{font-size:clamp(2.4rem,5.5vw,3.8rem)}
h2{font-size:clamp(1.7rem,3.5vw,2.5rem)}
h3{font-size:1.1rem}

.skip{position:absolute;top:-40px;left:16px;z-index:100;background:var(--surface);color:var(--ink);padding:.5rem 1rem;border-radius:8px;font-weight:700;transition:top .2s var(--ease)}
.skip:focus{top:8px}

/* ---------- nav ---------- */
.nav{
  position:sticky;top:0;z-index:50;
  display:flex;align-items:center;justify-content:space-between;gap:1rem;
  padding:.7rem clamp(1rem,4vw,2.5rem);
  background:rgba(255,249,246,.82);
  backdrop-filter:blur(16px) saturate(160%);
  -webkit-backdrop-filter:blur(16px) saturate(160%);
  border-bottom:1px solid var(--line);
}
.nav .brand{display:flex;align-items:center;gap:.55rem;font-family:"Varela Round";font-size:1.15rem;color:var(--ink)}
.nav .brand svg{width:34px;height:34px}
.nav nav{display:flex;gap:.25rem}
.nav nav a{
  display:inline-block;padding:.5rem .85rem;border-radius:10px;
  color:var(--ink-soft);font-weight:700;font-size:.9rem;
  transition:background .18s var(--ease),color .18s var(--ease);
}
.nav nav a:hover{background:#F5E6E2;color:var(--ink)}
a:focus-visible,button:focus-visible{outline:3px solid var(--accent);outline-offset:2px;border-radius:6px}

/* ---------- hero ---------- */
.hero{
  display:grid;grid-template-columns:1fr 1fr;gap:clamp(2rem,5vw,4rem);align-items:center;
  max-width:1120px;margin:0 auto;
  padding:clamp(3rem,8vw,5rem) clamp(1rem,4vw,2.5rem);
  min-height:calc(100dvh - 60px);
}
.hero-copy{display:flex;flex-direction:column;gap:1rem}
.hero-copy h1 em{font-style:normal;color:var(--accent-deep)}
.hero-copy .tag{font-size:1.1rem;color:var(--ink-soft);max-width:38ch}
.hero-cta{display:flex;gap:.75rem;flex-wrap:wrap;margin-top:.5rem}
.hero-chips{display:flex;gap:.5rem;flex-wrap:wrap;margin-top:.25rem}
.chip{
  display:inline-flex;align-items:center;gap:.35rem;
  padding:.45rem .9rem;border-radius:12px;
  background:var(--surface);border:1px solid var(--line);
  font-size:.85rem;font-weight:700;color:var(--ink-soft);
}
.chip svg{width:16px;height:16px;color:var(--accent)}

.hero-visual{position:relative;display:grid;place-items:center;min-height:320px}
.hero-visual .blob{
  position:absolute;width:75%;aspect-ratio:1;
  border-radius:44% 56% 52% 48% / 48% 44% 56% 52%;
  background:var(--line);opacity:.5;
}
.hero-visual img{
  position:relative;z-index:2;width:min(78%,400px);
  filter:drop-shadow(0 18px 28px rgba(0,0,0,.1));
}

/* ---------- buttons ---------- */
.btn{
  display:inline-flex;align-items:center;gap:.5rem;
  padding:.75rem 1.5rem;border-radius:14px;
  font-weight:800;font-size:.95rem;cursor:pointer;border:none;
  transition:transform .15s var(--ease),box-shadow .15s var(--ease),background .15s var(--ease);
}
.btn:active{transform:scale(.96)}
.btn-primary{background:var(--accent);color:#fff;box-shadow:0 4px 14px -4px rgba(212,90,80,.35)}
.btn-primary:hover{background:var(--accent-deep);transform:translateY(-1px);box-shadow:0 6px 20px -4px rgba(212,90,80,.45)}
.btn-ghost{background:var(--surface);color:var(--ink);border:1px solid var(--line)}
.btn-ghost:hover{background:#F5E6E2;transform:translateY(-1px)}

/* ---------- sections ---------- */
.section{max-width:1120px;margin:0 auto;padding:clamp(3rem,7vw,5rem) clamp(1rem,4vw,2.5rem)}
.section h2{margin-bottom:1rem}
.section .sub{color:var(--ink-soft);max-width:50ch;margin-bottom:2rem;font-size:1.05rem}

/* ---------- about split ---------- */
.split{display:grid;grid-template-columns:1fr 1fr;gap:clamp(2rem,4vw,3rem);align-items:start}
.split-visual{border-radius:var(--radius);overflow:hidden;aspect-ratio:1}
.split-visual img{width:100%;height:100%;object-fit:cover}
.split-copy{display:flex;flex-direction:column;gap:1rem}
.split-copy p{color:var(--ink-soft)}
.split-copy strong{color:var(--ink)}
.stat-row{display:grid;grid-template-columns:repeat(3,1fr);gap:.75rem;margin-top:.75rem}
.stat{text-align:center;padding:.9rem .4rem;background:var(--surface);border-radius:14px;border:1px solid var(--line)}
.stat b{display:block;font-size:1.6rem;font-family:"Varela Round";color:var(--accent-deep)}
.stat span{font-size:.78rem;color:var(--ink-soft)}

/* ---------- photo grid ---------- */
.photo-grid{display:grid;grid-template-columns:repeat(auto-fill,minmax(220px,1fr));gap:1rem}
.photo-card{
  position:relative;border-radius:var(--radius);overflow:hidden;
  aspect-ratio:1;background:var(--line);
}
.photo-card img{
  width:100%;height:100%;object-fit:cover;
  transition:transform .4s var(--ease);
}
.photo-card:hover img{transform:scale(1.04)}

/* ---------- favorites ---------- */
.fav-grid{display:grid;grid-template-columns:repeat(auto-fill,minmax(240px,1fr));gap:1.25rem}
.fav-card{
  display:flex;flex-direction:column;gap:.6rem;
  padding:1.25rem;background:var(--surface);border-radius:var(--radius);
  border:1px solid var(--line);
  transition:transform .2s var(--ease),box-shadow .2s var(--ease);
}
.fav-card:hover{transform:translateY(-3px);box-shadow:0 8px 24px -8px rgba(0,0,0,.08)}
.fav-card img{width:100%;aspect-ratio:4/3;object-fit:cover;border-radius:12px}
.fav-card h3{font-family:"Varela Round"}
.fav-card p{font-size:.9rem;color:var(--ink-soft)}

/* ---------- timeline ---------- */
.timeline{display:flex;flex-direction:column}
.tl-row{
  display:grid;grid-template-columns:70px 1fr;gap:1rem;
  padding:1rem 0;border-bottom:1px solid var(--line);
}
.tl-row:last-child{border-bottom:none}
.tl-time{font-family:"Varela Round";font-size:.85rem;color:var(--accent-deep);font-weight:700;padding-top:.15rem}
.tl-row b{font-size:.95rem}
.tl-row p{font-size:.85rem;color:var(--ink-soft);margin-top:.1rem}

/* ---------- map preview ---------- */
.map-wrap{
  position:relative;border-radius:var(--radius);overflow:hidden;
  margin-bottom:2rem;border:1px solid var(--line);
  height:320px;
}
.map-wrap iframe{
  position:absolute;inset:0;width:100%;height:100%;
  border:none;
}
.map-label{
  position:absolute;bottom:12px;right:12px;z-index:2;
  display:inline-flex;align-items:center;gap:.4rem;
  padding:.5rem 1rem;border-radius:10px;
  background:rgba(255,255,255,.92);backdrop-filter:blur(8px);
  font-size:.85rem;font-weight:700;color:var(--ink);
  box-shadow:0 2px 8px rgba(0,0,0,.1);
  text-decoration:none;
}
.map-label:hover{background:#fff}
.map-label svg{width:16px;height:16px;color:var(--accent)}

/* ---------- visit ---------- */
.visit-grid{display:grid;grid-template-columns:1fr 1fr;gap:clamp(1.5rem,4vw,2.5rem)}
.visit-card{
  padding:1.5rem;background:var(--surface);border-radius:var(--radius);
  border:1px solid var(--line);display:flex;flex-direction:column;gap:1rem;
}
.visit-card h3{font-family:"Varela Round"}
.visit-card ul{list-style:none;display:flex;flex-direction:column;gap:.5rem}
.visit-card li{font-size:.9rem;color:var(--ink-soft);padding-left:1.1rem;position:relative}
.visit-card li::before{content:"";position:absolute;left:0;top:.55em;width:6px;height:6px;border-radius:50%;background:var(--accent)}

/* ---------- footer ---------- */
footer{
  text-align:center;padding:2.5rem clamp(1rem,4vw,2.5rem);
  border-top:1px solid var(--line);
  font-size:.85rem;color:var(--ink-soft);
}
footer svg{width:20px;height:20px;color:var(--accent);margin-bottom:.5rem}

/* ---------- entrance ---------- */
[data-enter]{
  opacity:0;transform:translateY(20px);
  transition:opacity .55s var(--spring),transform .55s var(--spring);
}
[data-enter].in{opacity:1;transform:translateY(0)}

@media(prefers-reduced-motion:reduce){
  [data-enter]{opacity:1;transform:none;transition:opacity .25s ease}
  .photo-card:hover img,.fav-card:hover{transform:none}
  .btn:active{transform:none}
}

@media(max-width:768px){
  .hero{grid-template-columns:1fr;min-height:auto;padding-top:2rem}
  .hero-visual{order:-1;min-height:220px}
  .split,.visit-grid{grid-template-columns:1fr}
  .nav nav{display:none}
  .photo-grid{grid-template-columns:repeat(2,1fr)}
  .fav-grid{grid-template-columns:1fr}
}
@media(max-width:480px){
  .photo-grid{grid-template-columns:1fr}
  .stat-row{grid-template-columns:repeat(2,1fr)}
  .tl-row{grid-template-columns:55px 1fr}
}
</style>
</head>
<body>

<a class="skip" href="#main">Skip to content</a>

<header class="nav">
  <a class="brand" href="#top">
    <svg viewBox="0 0 100 100" aria-hidden="true"><circle cx="50" cy="50" r="48" fill="#E8756A"/><circle cx="34" cy="42" r="6" fill="#3D2B26"/><circle cx="66" cy="42" r="6" fill="#3D2B26"/><ellipse cx="50" cy="62" rx="14" ry="10" fill="#FFE3DF"/><circle cx="50" cy="55" r="5" fill="#3D2B26"/><path d="M38 62 Q50 72 62 62" stroke="#3D2B26" stroke-width="3" fill="none" stroke-linecap="round"/></svg>
    モネ
  </a>
  <nav aria-label="Sections">
    <a href="#about">About</a>
    <a href="#gallery">Photos</a>
    <a href="#favorites">Favorites</a>
    <a href="#visit">Visit</a>
  </nav>
</header>

<main id="main" data-enter>

  <section class="hero" id="top" aria-labelledby="hero-title">
    <div class="hero-copy">
      <h1 id="hero-title">This is <em>モネ</em>.</h1>
      <p class="tag">Pom Poodle mix. 8&ndash;10 lbs of fluff. Lives in Yokohama. Eats her own poop. Barks at everyone like you&rsquo;re an intruder.</p>
      <div class="hero-cta">
        <a class="btn btn-primary" href="#gallery">See photos</a>
        <a class="btn btn-ghost" href="#visit">Say hi</a>
      </div>
      <div class="hero-chips">
        <span class="chip"><svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" aria-hidden="true"><path d="M12 21s-7-4.5-9.5-9A5.5 5.5 0 0 1 12 6a5.5 5.5 0 0 1 9.5 6c-2.5 4.5-9.5 9-9.5 9z"/></svg>Pom Poodle</span>
        <span class="chip"><svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" aria-hidden="true"><path d="M20 10c0 6-8 11-8 11S4 16 4 10a8 8 0 0 1 16 0z"/><circle cx="12" cy="10" r="3"/></svg>Yokohama</span>
        <span class="chip"><svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" aria-hidden="true"><circle cx="12" cy="12" r="9"/><path d="M8 14s1.5 2 4 2 4-2 4-2"/><path d="M9 9h.01M15 9h.01" stroke-width="3"/></svg>Very small</span>
      </div>
    </div>
    <div class="hero-visual">
      <div class="blob" aria-hidden="true"></div>
      <img src="/img/monet-kitty-hat-cutout.png" alt="モネ wearing a Hello Kitty hood" loading="eager" decoding="async">
    </div>
  </section>

  <section class="section" id="about" aria-labelledby="about-title">
    <h2 id="about-title">About モネ</h2>
    <div class="split">
      <div class="split-visual">
        <img src="/img/monet-beige-sweater.png" alt="モネ in a beige sweater" loading="lazy" decoding="async">
      </div>
      <div class="split-copy">
        <p><strong>Breed:</strong> Pomeranian &times; Poodle mix. The poodle gave the curls, the pom gave the fluff and the attitude.</p>
        <p><strong>Age:</strong> 1&ndash;2 years old. Still a puppy, just with more opinions.</p>
        <p><strong>Weight:</strong> 8&ndash;10 lbs. All fluff, no structure.</p>
        <p><strong>Home:</strong> Yokohama, Japan. Spends most days at the bay, the park, and wherever there&rsquo;s a treat hiding.</p>
        <p><strong>Personality:</strong> Barks at every single person who walks by. Treats them all as intruders. Also eats her own poop. You cannot fix both at once.</p>
        <div class="stat-row">
          <div class="stat"><b>8&ndash;10</b><span>lbs of fluff</span></div>
          <div class="stat"><b>1&ndash;2</b><span>years old</span></div>
          <div class="stat"><b>&infin;</b><span>leaf sessions</span></div>
        </div>
      </div>
    </div>
  </section>

  <section class="section" id="gallery" aria-labelledby="gallery-title">
    <h2 id="gallery-title">Photos</h2>
    <div class="photo-grid">
      <div class="photo-card"><img src="/img/monet-kitty-hat.png" alt="Hello Kitty hood" loading="lazy" decoding="async"></div>
      <div class="photo-card"><img src="/img/monet-bowtie.png" alt="Bow tie" loading="lazy" decoding="async"></div>
      <div class="photo-card"><img src="/img/monet-knit-vest.png" alt="Knit vest" loading="lazy" decoding="async"></div>
      <div class="photo-card"><img src="/img/monet-loafing.png" alt="Loafing" loading="lazy" decoding="async"></div>
      <div class="photo-card"><img src="/img/monet-outdoor-coat.png" alt="Outdoor coat" loading="lazy" decoding="async"></div>
      <div class="photo-card"><img src="/img/monet-side-profile.png" alt="Side profile" loading="lazy" decoding="async"></div>
      <div class="photo-card"><img src="/img/monet-with-human.png" alt="With their human" loading="lazy" decoding="async"></div>
      <div class="photo-card"><img src="/img/monet-hello-kitty-onesie.png" alt="Hello Kitty onesie" loading="lazy" decoding="async"></div>
      <div class="photo-card"><img src="/img/monet-beige-sweater.png" alt="Beige sweater" loading="lazy" decoding="async"></div>
    </div>
  </section>

  <section class="section" id="favorites" aria-labelledby="fav-title">
    <h2 id="fav-title">What she&rsquo;s into</h2>
    <div class="fav-grid">
      <div class="fav-card">
        <img src="/img/monet-loafing.png" alt="モネ in leaves" loading="lazy" decoding="async">
        <h3>Leaves</h3>
        <p>Falls in Yokohama are her season. Bats at them, rolls in them, chews one and spits it out. Full commitment.</p>
      </div>
      <div class="fav-card">
        <img src="/img/monet-with-human.png" alt="モネ barking at a human" loading="lazy" decoding="async">
        <h3>Barking at everyone</h3>
        <p>Every person. Every dog. The mailman. The wind. She&rsquo;s the neighborhood&rsquo;s security system.</p>
      </div>
      <div class="fav-card">
        <img src="/img/monet-knit-vest.png" alt="モネ eating" loading="lazy" decoding="async">
        <h3>Treats</h3>
        <p>Training treats, bathhouse treats, treats that appear from nowhere. All of them. All the time.</p>
      </div>
      <div class="fav-card">
        <img src="/img/monet-outdoor-coat.png" alt="モネ at the bay" loading="lazy" decoding="async">
        <h3>The bay</h3>
        <p>Wades in, shakes out, repeats. The water line is her office.</p>
      </div>
      <div class="fav-card">
        <img src="/img/monet-kitty-hat.png" alt="モネ in a Hello Kitty hood" loading="lazy" decoding="async">
        <h3>Getting dressed up</h3>
        <p>Will sit perfectly still for 10 seconds while you put on a bow tie. Then immediately shakes it off.</p>
      </div>
      <div class="fav-card">
        <img src="/img/monet-side-profile.png" alt="モネ loafing" loading="lazy" decoding="async">
        <h3>Being small</h3>
        <p>8&ndash;10 lbs and she acts like 40. Squeezes into every pocket-sized space in the house.</p>
      </div>
    </div>
  </section>

  <section class="section" id="daily" aria-labelledby="day-title">
    <h2 id="day-title">A typical day</h2>
    <div class="timeline">
      <div class="tl-row">
        <span class="tl-time">07:30</span>
        <div><b>Zoomies</b><p>Whole apartment. No reason. No destination.</p></div>
      </div>
      <div class="tl-row">
        <span class="tl-time">08:00</span>
        <div><b>Breakfast</b><p>Treats first. Kibble second. Negotiable.</p></div>
      </div>
      <div class="tl-row">
        <span class="tl-time">10:00</span>
        <div><b>Park</b><p>Sniff everything. Bark at every person. Chase one leaf and commit to it.</p></div>
      </div>
      <div class="tl-row">
        <span class="tl-time">13:00</span>
        <div><b>Leaf session</b><p>If there are leaves, it&rsquo;s a leaf session. If not, she&rsquo;s disappointed.</p></div>
      </div>
      <div class="tl-row">
        <span class="tl-time">14:00</span>
        <div><b>Nap</b><p>One sunbeam. Full body. 45 minutes minimum.</p></div>
      </div>
      <div class="tl-row">
        <span class="tl-time">17:00</span>
        <div><b>Bay walk</b><p>Wades in. Shakes out. Barks at the water. Repeats.</p></div>
      </div>
      <div class="tl-row">
        <span class="tl-time">20:00</span>
        <div><b>Dinner + the incident</b><p>Dinner. Then the poop-eating incident. It always happens.</p></div>
      </div>
      <div class="tl-row">
        <span class="tl-time">21:30</span>
        <div><b>Pets + good girl</b><p>End of shift. Everyone pretends the incident didn&rsquo;t happen.</p></div>
      </div>
    </div>
  </section>

  <section class="section" id="visit" aria-labelledby="visit-title">
    <h2 id="visit-title">Come say hi</h2>
    <div class="map-wrap">
      <iframe src="https://www.google.com/maps?q=Yokohama&z=12&output=embed" title="Map of Yokohama" loading="lazy" referrerpolicy="no-referrer-when-downgrade"></iframe>
      <a class="map-label" href="https://www.google.com/maps/place/Yokohama" target="_blank" rel="noopener">
        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" aria-hidden="true"><path d="M20 10c0 6-8 11-8 11S4 16 4 10a8 8 0 0 1 16 0z"/><circle cx="12" cy="10" r="3"/></svg>
        Open in Google Maps
      </a>
    </div>
    <div class="visit-grid">
      <div class="visit-card">
        <h3>Where</h3>
        <ul>
          <li>Yokohama. The park, the bay, or her front door.</li>
          <li>Best time: mornings, before nap. She&rsquo;s zoomiest at 07:30.</li>
        </ul>
      </div>
      <div class="visit-card">
        <h3>Know before you go</h3>
        <ul>
          <li>She will bark at you. This is normal. It means hi.</li>
          <li>Treats: always yes. Bring some.</li>
          <li>Let her come to you. She&rsquo;s 8&ndash;10 lbs but she barks like 40.</li>
          <li>Don&rsquo;t comb the curls unless you&rsquo;re the human.</li>
          <li>Leave happy. Everybody leaves happy.</li>
        </ul>
      </div>
    </div>
  </section>

</main>

<footer>
  <svg viewBox="0 0 24 24" fill="currentColor" aria-hidden="true"><path d="M12 21s-7-4.5-9.5-9A5.5 5.5 0 0 1 12 6a5.5 5.5 0 0 1 9.5 6c-2.5 4.5-9.5 9-9.5 9z"/></svg>
  <div>モネ &middot; Pom Poodle mix &middot; Yokohama &middot; <span id="year">2026</span></div>
</footer>

<script>
(function(){
  var y=document.getElementById("year");
  if(y){y.textContent=new Date().getFullYear();}
  var m=document.querySelector("[data-enter]");
  if(m){
    requestAnimationFrame(function(){
      requestAnimationFrame(function(){
        m.classList.add("in");
      });
    });
  }
})();
</script>
</body>
</html>
"##;

#[cfg(test)]
mod tests {
    use super::PAGE;
    #[test]
    fn page_sanity() {
        assert!(PAGE.starts_with("<!DOCTYPE html>"));
        assert!(PAGE.contains("モネ"));
        assert!(PAGE.contains("Yokohama"));
        assert!(PAGE.contains("Pom Poodle"));
        assert!(PAGE.contains("eats her own poop"));
        assert!(PAGE.contains("Barking at everyone"));
        assert!(PAGE.contains("prefers-reduced-motion"));
        assert!(PAGE.contains("backdrop-filter"));
        assert!(PAGE.contains("data-enter"));
    }
}
