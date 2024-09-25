import Logo from './assets/react.svg'
import { useEffect, useState } from 'react' 
import './App.css'

function App() {

  const [posts, setPosts] = useState([]);


   useEffect(() => {
      fetch('https://jsonplaceholder.typicode.com/posts?_limit=1')
         .then((response) => response.json())
         .then((data) => {
	    setPosts(data);
            console.log(data);
         })
         .catch((err) => {
            console.log(err.message);
         });
   }, []);

  return (
    <>
      <div className='ui'>
	<div className='top'>
	  <img src={Logo} alt="Mangoloader Logo"/> 
	  <h1>Mangoloader</h1>
	</div>
	<div className='middle'>
              {posts.map((post) => {
         return (
            <div className="post-card" key={post.id}>
               <h2 className="post-title">{post.title}</h2>
               <p className="post-body">{post.body}</p>
               <div className="button">
               <div className="delete-btn">Delete</div>
               </div>
            </div>
          );
	 })}
	</div>
      </div>
    </>
  )
}

export default App
