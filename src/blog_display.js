import { useState, useEffect } from "react";
import { useLocation } from 'react-router-dom';
import Markdown from "markdown-to-jsx";
import Navbar from "./components/navbar";

import "./blog.css"

const mdoptions={}

function Blog() {
  let loc = useLocation();
  const [postContent, setPostContent] = useState('');

  useEffect(() => {
    import('./Content' + loc.pathname)
      .then(res =>
        fetch(res.default)
          .then(response => response.text())
          .then(response => setPostContent(response))
          .catch(err => console.log(err))
      )
  }, [loc.pathname])

  return (
    <div>
    <Navbar />
      <div className="article-wrapper">
        <article>
          <main>
            <Markdown options={mdoptions}>
              {postContent}
            </Markdown>
          </main>
        </article>
      </div>
    </div>
  )
}

export default Blog;