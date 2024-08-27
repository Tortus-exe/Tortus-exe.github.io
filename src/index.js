import { BrowserRouter, Routes, Route } from "react-router-dom";
import React from 'react';
import ReactDOM from 'react-dom/client';
import './index.css';
import App from './App';
import About from './About';
import Blog from './blog_display'
import CategoryListing from './category_listing'
import reportWebVitals from './reportWebVitals';

const root = ReactDOM.createRoot(document.getElementById('root'));
root.render(
    <BrowserRouter>
      <Routes>
        <Route path="/">
          <Route index element={<App />} />
          <Route path="about" element={<About />} />
          <Route path="BQN">
            <Route index element={<CategoryListing folder="BQN" />}/>
            <Route path="*" element={<Blog />} />
          </Route>
          <Route path="Cantonese">
            <Route index element={<CategoryListing folder="Canto" />}/>
            <Route path="*" element={<Blog />} />
          </Route>
          <Route path="*" element={<App />} />
        </Route>
      </Routes>
    </BrowserRouter>
);

    /* <React.StrictMode> */
    /*     <App /> */
    /* </React.StrictMode> */
// If you want to start measuring performance in your app, pass a function
// to log results (for example: reportWebVitals(console.log))
// or send to an analytics endpoint. Learn more: https://bit.ly/CRA-vitals
reportWebVitals();
