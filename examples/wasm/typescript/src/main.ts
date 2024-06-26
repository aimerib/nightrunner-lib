import './style.css'
import data from './data.json';
import { NightRunner } from '@nightrunner/nightrunner_lib'


// Load the NightRunner library.
// The NightRunner class expects stringified JSON data. For more information
// on valid fields, see the NightRunner documentation.
const nr: NightRunner = new NightRunner(JSON.stringify(data));
const app = document.querySelector<HTMLDivElement>('#app')!

const first_room_text = nr.first_room_text();

app.innerHTML = `
<h1>Nightrunner example output result</h1>
<p class="display">${first_room_text.message}</p>
`

let result = nr.parse("look");
let message = result["messageType"] == "look" ? result.data : "";
app.innerHTML += `
  <br />
  <h2>Input:</h2>
  <pre>look</pre>
  <h3>Event message</h3>
  <p class="display">${message}</p>
  <h3>Payload</h3>
  <pre>${JSON.stringify(result, null, 2)}</pre>
`

result = nr.parse("look at item1");
message = result["messageType"] == "look" ? result.data : "";
app.innerHTML += `
  <br />
  <h2>Input:</h2>
  <pre>look at item1</pre>
  <h3>Event message</h3>
  <p class="display">${message}</p>
  <h3>Payload</h3>
  <pre>${JSON.stringify(result, null, 2)}</pre>
  `

result = nr.parse("south");
console.log(result)
message = result["messageType"] == "event_success" ? result.data.message : "";
app.innerHTML += `
  <br />
  <br />
  <h2>Input:</h2>
  <pre>south</pre>
  <h3>Event message</h3>
  <p class="display">${message}</p>
  <h3>Payload</h3>
  <pre>${JSON.stringify(result, null, 2)}</pre>
  `
