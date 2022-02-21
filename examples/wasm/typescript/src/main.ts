import './style.css'
import data from './data.json';
import init, {NightRunner} from '@nightrunner/nightrunner_lib'

await init(); // Vite has global async/await support on by default.

// Load the NightRunner library.
// The NightRunner class expects stringified JSON data. For more information
// on valid fields, see the NightRunner documentation.
const nr: NightRunner = await new NightRunner(JSON.stringify(data));
const app = document.querySelector<HTMLDivElement>('#app')!

const first_room_text = nr.first_room_text();

app.innerHTML = `
<h1>Nightrunner output result</h1>
<p class="display">${first_room_text.message}</p>
`

let result = JSON.parse(nr.parse("look"));
let message = result["ok"] ? result.ok.look : result["error"];
app.innerHTML += `
  <br />
  <h2>Input:</h2>
  <pre>look</pre>
  <h3>Event message</h3>
  <p class="display">${message}</p>
  <h3>Payload</h3>
  <pre>${JSON.stringify(result)}</pre>
`

result = JSON.parse(nr.parse("look at item1"));
message = result["ok"] ? result.ok.look : result["error"];
app.innerHTML += `
  <br />
  <h2>Input:</h2>
  <pre>look at item1</pre>
  <h3>Event message</h3>
  <p class="display">${message}</p>
  <h3>Payload</h3>
  <pre>${JSON.stringify(result)}</pre>
  `

result = JSON.parse(nr.parse("south"));
console.log(result)
message = result["ok"] ? result.ok.event_success.message : result["error"];
app.innerHTML += `
  <br />
  <br />
  <h2>Input:</h2>
  <pre>look at item1</pre>
  <h3>Event message</h3>
  <p class="display">${message}</p>
  <h3>Payload</h3>
  <pre>${JSON.stringify(result)}</pre>
  `