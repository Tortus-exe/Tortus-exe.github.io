import { Link } from "react-router-dom";
import categories from "./Content/categories.json";
import React, { useState } from 'react';
import './App.css';

const art = [
  `
  hello
  `,
  `
  goodbye
  `,
  `
  no
  `,
  `⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⢿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠿⠛⢉⠀⡀⠀⠘⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠟⢁⠐⡈⠄⠂⠄⠀⠀⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡿⠁⡐⢀⠂⡐⢈⠐⠀⠀⢀⣿⣿⣿⣿⣿⣿⣿⣿⣿⡏⢸⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣿⡿⢿⣿⡇⠐⡀⢂⠐⡀⢂⠀⠀⠀⣼⣿⣿⣿⣿⣿⣿⣿⣿⡿⠀⠈⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⠏⠀⢀⣿⡇⠠⠐⡀⢂⠐⡀⠀⠀⢰⣿⣿⣿⣿⣿⣿⣿⣿⣿⠃⠀⠀⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⡿⠃⠀⠀⣾⣿⠃⢀⠡⠐⡀⢂⠀⠀⢀⣿⣿⣿⣿⣿⣿⣿⣿⣿⠇⠀⠀⠀⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⡟⠁⢀⠀⢸⣿⡿⠀⠄⠂⡁⠐⡀⠀⠀⠘⠉⠀⠀⠀⠀⠉⠛⠛⠛⠀⠀⠀⠀⢸⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿
⣿⣿⡟⠀⡀⢂⠀⢿⣿⠃⠐⡈⢄⠐⠠⠀⠀⠀⣠⣶⣿⣿⣿⣷⡄⠀⠀⠀⠀⠀⠀⠀⠈⠉⠛⠛⢿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿
⣿⡿⠀⡐⢀⠂⠄⡈⠁⢀⠂⡐⢀⠀⠂⠀⢠⣾⣿⣿⣿⣿⣿⣿⡇⠀⠀⢀⠀⠠⠀⠄⢂⠐⠠⠀⠾⠿⠿⢿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿
⣿⡇⢀⠐⠀⢈⣐⠀⡁⢂⠐⡀⠂⠌⠀⠀⢸⣿⣿⣿⣿⣿⣿⡟⠀⠀⡈⠄⠈⠄⠡⠈⠀⢈⠀⠠⢁⠌⡒⣧⡨⢻⣿⣿⣿⣿⣿⣿⣿⣿
⣿⡇⠂⢀⣌⠀⣿⣄⣐⣠⣴⣶⣌⠐⠠⠀⠈⠻⣿⣿⡿⠿⠋⠀⠀⠄⠀⠀⠀⠀⠀⠀⠀⠀⢠⣶⡅⢂⠰⠙⡓⢸⣿⣿⣿⣿⣿⣿⣿⣿
⣿⡇⣱⣿⠃⠠⢹⣿⣿⣿⣿⣿⡏⠠⢁⠐⠠⢀⡀⢠⠓⠀⠄⠈⠐⢈⠐⡀⠌⠠⡀⠄⠠⠀⠉⢛⠃⠂⠀⢀⠀⢼⣿⡿⠋⠉⠻⣿⣿⣿
⣿⣿⣿⣿⣆⠀⢸⣿⣿⣿⣿⠿⠁⠂⠀⠈⠀⡀⣟⡀⠂⢌⠄⠃⠌⠄⠒⠠⠌⠡⢐⠈⡡⠀⠀⢠⣈⣡⠀⠂⠀⣘⣛⠀⠀⠀⠀⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣿⠿⠟⠁⠀⢀⠠⠐⣈⠐⡀⢯⡆⢈⠄⡉⠡⢈⠈⠄⠁⠌⠐⡈⠰⠀⠀⠀⣸⣿⣿⣧⠀⢠⣿⠋⠀⣠⢠⣾⣿⣿⣿
⣿⣿⣿⣿⣿⠟⠩⠐⠈⡀⠀⠀⠀⢂⠡⠄⢂⠁⠫⠜⡀⢂⠄⢁⠀⠄⢂⠈⡀⠡⠀⣁⠀⠀⣴⣿⣿⣿⣿⡇⠠⠁⠠⠾⠟⠏⠉⠛⣿⣿
⣿⣿⣿⢟⠕⡈⠤⢡⣾⣿⣦⡀⠀⠀⠀⠀⠀⠀⠁⠀⡐⠀⠌⠠⢈⠐⡀⠂⠀⠄⠸⣏⣴⣾⣿⣿⣿⡿⠋⠀⠀⣤⣄⠀⠀⠀⠠⠀⢸⣿
⣿⣿⠏⡞⠠⡐⢠⣿⣿⣿⣿⣿⣿⠀⠀⡀⠁⠀⠂⠀⠀⠀⠌⡐⢀⠂⠄⠁⠀⠂⠰⣿⣿⣿⣿⣿⠟⠀⠀⠂⠀⣿⣿⣀⣶⣄⣁⣀⣼⣿
⣿⣿⠀⠇⢡⠐⠸⣿⣿⣿⣿⣿⣿⡆⠀⠀⠀⠂⠀⠀⠀⠀⢂⢐⠠⠈⠠⠀⠀⠀⠈⢿⣿⣿⡿⠁⠀⠀⠠⠐⠀⣿⣿⣿⣿⣿⣿⣿⣿⣿
⣿⣿⠀⠀⠂⡌⢐⠈⠻⣿⠋⡐⠈⢻⡀⠀⠀⡀⠈⠀⠄⠁⢂⠈⠄⠠⠁⠌⡀⠂⠀⠘⣿⡟⠀⠀⠈⠐⠀⠀⣸⣿⣿⣿⣿⣿⣿⣿⣿⣿
⣿⣿⡄⠀⠡⠐⣈⠐⠀⢧⡀⠀⢀⡜⠀⠀⠀⢀⠀⠂⠀⠠⠀⠌⠠⠁⠌⡐⠀⠁⣦⠀⠘⠀⠀⡀⠁⠀⡀⢠⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣄⠀⠀⠀⠀⢀⣾⣿⣿⣿⠁⠀⣤⠀⠀⠈⠄⠀⡀⠄⠈⠄⡁⢂⠐⠀⣼⣿⣆⠀⠀⠄⠀⠠⠀⢀⣾⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣶⣶⣶⣿⣿⣿⣿⠃⢀⢰⣿⣆⠀⠀⠀⠄⠀⠀⡀⠂⡐⠠⠈⢠⣿⣿⣿⡆⠀⠀⠀⠀⠀⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡏⡐⢬⢎⡭⣝⠂⠀⠀⠀⠀⠁⠀⢀⠀⠁⠀⣸⣿⣿⣿⣷⣦⣤⣶⡀⢰⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡀⠁⠠⢉⠞⡱⢎⠣⠀⠀⠀⠀⠀⠁⠀⠀⠀⠀⢸⣿⣿⣿⣿⣿⣿⣿⣿⣼⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿`,
]

function AsciiPreview(data) {
  return (
    <p className="asciiArt" key={data}>{art[data]}</p>
  );
}

function MakeButton(data, num, activeOption, setActive) {
    let active = activeOption === num
    return (
      <Link to={data.uri}>
        <button key={num} className={"mainScreenButton" + (active ? " active" : "")}
          onMouseOver={() => {setActive(num)}}
          onMouseLeave={() => {setActive(3)}}>
              <div className={"buttonText" + (active ? " active" : "")}>
                  {data.title}
              </div>
        </button>
      </Link>
    );
}

function App() {
  let options = categories.categories;
  const [activeOption, setActiveOption] = useState(options.length);

  return (
    <div className="App">
      <div className="previewDiv">
        {AsciiPreview(activeOption)}
      </div>
      <div className="selectionsDiv">
        {options.map((a, b) => MakeButton(a, b, activeOption, setActiveOption), this)}
      </div>
    </div>
  );
}

export default App;
