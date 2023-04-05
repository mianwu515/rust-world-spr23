import React, { useState } from "react";
import ReactDOM from "react-dom";

function App() {
  const [message, setMessage] = useState("");
  const [response, setResponse] = useState("");

  const handleSubmit = async (event) => {
    event.preventDefault();

    const requestOptions = {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ message }),
    };

    const res = await fetch("/chat", requestOptions);
    const data = await res.json();

    setResponse(data.response);
  };

  const handleChange = (event) => {
    setMessage(event.target.value);
  };

  return (
    <div>
      <form onSubmit={handleSubmit}>
        <label htmlFor="message">Message:</label>
        <input type="text" id="message" value={message} onChange={handleChange} />
        <button type="submit">Send</button>
      </form>
      <div>{response}</div>
    </div>
  );
}

ReactDOM.render(<App />, document.getElementById("root"));
