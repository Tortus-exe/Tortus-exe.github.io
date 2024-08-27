import { useState, useEffect } from 'react';
import { useLocation, Link } from 'react-router-dom';
import Navbar from './components/navbar';
import Markdown from 'markdown-to-jsx';

import "./category_listing.css";

function MakeEntry(data, num) {
  let loc = useLocation();

  return (
    <Link to={`${loc.pathname}/${data}`}>
      <button key={num} className="entryButton">
        {data}
      </button>
    </Link>
  );
}

function CategoryListing(k) {
  let loc = useLocation();
  const [listing, setListing] = useState([]);
  const [synopsis, setSynopsis] = useState('');

  useEffect(() => {
    import(`./Content${loc.pathname}/contents.json`)
      .then(res => setListing(res.contents))
      .catch(err => console.log(err))
  }, [loc.pathname])

  useEffect(() => {
    import('./Content' + loc.pathname + '/synopsis.md')
      .then(res =>
        fetch(res.default)
          .then(response => response.text())
          .then(response => setSynopsis(response))
          .catch(err => console.log(err))
      )
  }, [loc.pathname])

  return (
    <div>
      <Navbar />
      <div className="listingAndSynopsisContainer">
        <div className="articleListing">
          {listing.map((a, b) => MakeEntry(a, b))}
        </div>
        <div className="synopsis">
          <Markdown>
            {synopsis}
          </Markdown>
        </div>
      </div>
    </div>
  );
}

export default CategoryListing;