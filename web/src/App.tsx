import Logo from './assets/react.svg'
import { useEffect, useState } from 'react' 
import './App.css'


function App() {


  const [test, setTest] = useState<{ test: string }[]>([]);

   useEffect(() => {
     fetch('/api/test')
         .then((response) => response.json())
         .then((data) => {
	    setTest(data);
            console.log(data);
         })
         .catch((err) => {
            console.log(err);
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
	  {test.length === 0 && "Loading..."}
	  {test.length !== 0 && <h2>{test[0].test}</h2>}
	</div>
      </div>
    </>
  )
}

export default App
