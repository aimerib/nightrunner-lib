import {
  FormEvent,
  MutableRefObject,
  useEffect,
  useRef,
  useState,
} from "react";
import { NightRunner } from "@nightrunner/nightrunner_lib";
import "./App.css";

// *** Types ***
//
// Types to make the code more readable. This is not necessary, but it helps
// to understand the code, and since this is typescript, it's a good habit.

/**
 * Props for the App component.
 * @param {NightRunner} engine - The NightRunner instance.
 */
type AppProps = {
  engine: NightRunner;
};

/**
 * ActionResult is a union type of the possible results of an action.
 * An action is a command the player types and is parsed by NightRunner.
 *
 * The messageType is a string that describes the type of the result in
 * the context of an action.
 *
 * @param {string} messageType - The type of the result from the action as interpreted by NightRunner.
 * @param {string} data - When the return messageType from the parser is anything other than an `EventSuccess`, the value will be a string.
 */
type ActionResult = {
  messageType:
    | "look"
    | "inventory"
    | "dropItem"
    | "newItem"
    | "quit"
    | "help"
    | "subjectNoEvent";
  data: string;
};

/**
 * When the action executed by the player triggers an event, the event result is returned.
 * @param {string} messageType - The type of the result from the action as interpreted by NightRunner.
 * @param {EventSuccess} data -  When the return messageType from the parser is an `EventSuccess`, the value will be an object.
 */
type EventResult = {
  messageType: "eventSuccess";
  data: EventSuccess;
};

/**
 * @typedef { {firstName: string, lastName: string} } BrokenName
 */

/**
 * An EventSuccess type is an object that contains the result of an event.
 * It contains a message field that is a string, with all message_parts concatenated
 * together and can be used for simple layouts that don't require any special
 * formatting.
 * It also contains a message_parts field that is an object with the three main parts
 * of the message returned by NightRunner.
 * templated_words is an array of strings that are marked as template words using the
 * `{}` syntax in the data provided for NightRunner during initialization. The
 * templated words are replaced with the values contained in the `{}` syntax and also
 * added to this array. These words can be used to identify the words that were replaced
 * and can be used for highlighting the words in the display text. The only words that
 * are replaced are items and subjects currently in the player's current location.
 *
 * @param {string} message - The message returned by NightRunner corresponding to the event
 * result.
 * @param {string[]} templated_words - An array of strings that are marked as template words
 * using the `{}` syntax in the data provided for NightRunner during initialization.
 * @param {MessageParts} message_parts - An object with the three main parts of an event message
 * returned by NightRunner.
 */
type EventSuccess = {
  message: string;
  message_parts: MessageParts;
  templated_words: string[];
};

/**
 * A successful event message will contain the three main parts of the message.
 *
 * @param {string} room_text - This will be the text of the corresponding narrative active.
 * @param {string} event_text - Extra messages that might be returned from the event in adition to the narrative.
 * @param {string} exits - The exits of the current room, their directions, and a description of the room they lead to.
 *
 */
type MessageParts = {
  room_text: string;
  event_text: string;
  exits: string;
};

/**
 * A successful result will be either a string corresponding to the action result
 * or an EventSuccess object corresponding to the event result.
 */
type ResultOk = ActionResult | EventResult;

type ResultError = {
  message: string;
};

/**
 * An error returned by NightRunner will have a string message that can be displayed
 * to the player. We can't anotate an error type in a catch block, so we have to
 * help typescript know that the error contains a message field.
 * We can do this with a type guard function.
 * For more information check the documentation for [Type Guards]{@link (https://www.typescriptlang.org/docs/handbook/advanced-types.html#type-guards-and-type-assertions)}
 * @param x - The error object returned by NightRunner.
 * @returns boolean - True if the error object has a message field of type string.
 */
const isError = (x: any): x is ResultError => {
  return typeof x.message === "string";
};

/**
 * @returns JSX.Element - An empty div always at the bottom of the event message container.
 * This is used to make the event message container scroll to the bottom when more text is added.
 */
const AlwaysScrollToBottom = () => {
  const elementRef = useRef() as MutableRefObject<HTMLDivElement>;
  useEffect(() => elementRef.current.scrollIntoView());
  return <div ref={elementRef} />;
};

function App({ engine }: AppProps) {
  const firstRoomData: EventSuccess = engine.first_room_text();
  const [input, setInput] = useState("");
  const [roomText, setRoomText] = useState(
    firstRoomData.message_parts.room_text
  );
  const [eventText, setEventText] = useState<string[]>([]);
  const [exits, setExits] = useState(firstRoomData.message_parts.exits);
  const [message, setMessage] = useState(firstRoomData.message);
  const [currentIndex, setCurrentIndex] = useState(0);

  const submitAction = (e: FormEvent) => {
    e.preventDefault();
    try {
      let result: ResultOk = engine.parse(input);
      parseResult(result);
    } catch (e) {
      if (isError(e)) {
        eventText.push(e.message);
      }
    }
    setInput("");
  };
  const parseResult = (result: ResultOk) => {
    switch (result.messageType) {
      case "look":
      case "inventory":
      case "dropItem":
      case "newItem":
      case "subjectNoEvent":
        if (eventText.length > 0 && result.data.length > 0) {
          eventText.push("\n");
        }
        if (result.data.length > 0) {
          eventText.push(result.data);
        }
        break;
      case "help":
        alert(result.data);
        break;
      case "eventSuccess":
        const {
          room_text: new_room_text,
          exits: new_exits,
          event_text: new_event_text,
        } = result.data.message_parts;
        setRoomText(new_room_text);
        setExits(new_exits);
        setMessage(result.data.message);
        if (new_event_text.length > 0) {
          setEventText([new_event_text]);
        } else {
          setEventText([]);
        }
        break;
    }
    // This is here so you can see the result of the action in the console for
    // learning purposes.
    console.log(result);
  };

  const renderEventText = () => {
    return eventText.map((text, i) => {
      return <div key={i}>{text}</div>;
    });
  };

  const renderSeparateAreas = () => {
    return (
      <div>
        <h1>Using each message part in their own area</h1>
        <div className="centered">
          <div>Room message</div>
          <pre className="room-message display-area">{roomText}</pre>
          <div>Event Message</div>
          <pre className="event-message display-area">
            <div>
              {renderEventText()}
              <AlwaysScrollToBottom />
            </div>
          </pre>
          <div>Exits</div>
          <pre className="exits display-area">{exits}</pre>
        </div>
      </div>
    );
  };

  const renderSingleArea = () => {
    return (
      <div>
        <h1>Using separate message parts in one single area</h1>
        <div className="centered">
          <pre className="display-area single-area">
            <div>{roomText}</div>
            <br />
            <div>{renderEventText()}</div>
            <br />
            <div>{exits}</div>
            <AlwaysScrollToBottom />
          </pre>
        </div>
      </div>
    );
  };

  const renderMessageNoParts = () => {
    return (
      <div>
        <h1>Using the full message without extra parts</h1>
        <div className="centered">
          <pre className="display-area single-area">
            <div>{message + "\n\n" + eventText.join("\n")}</div>
            <AlwaysScrollToBottom />
          </pre>
        </div>
      </div>
    );
  };

  let test = [
    renderSingleArea(),
    renderSeparateAreas(),
    renderMessageNoParts(),
  ];

  return (
    <div className="App">
      <header className="App-header">
        {test[currentIndex]}
        {/* {showSplitDisplay ? renderSeparateAreas() : renderSingleArea()} */}
        <form onSubmit={submitAction}>
          <input
            type="text"
            value={input}
            onChange={(e) => setInput(e.target.value)}
            required
          />
          <button type="submit">Send</button>
        </form>
        <button
          style={{ marginTop: "15px" }}
          onClick={() => {
            if (currentIndex < test.length - 1) {
              setCurrentIndex(currentIndex + 1);
            } else {
              setCurrentIndex(0);
            }
          }}
        >
          Toggle message parts display
        </button>
      </header>
    </div>
  );
}

export default App;
