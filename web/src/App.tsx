import Logo from './assets/react.svg'
import { useEffect, useState } from 'react' 
import './App.css'


function App() {

  const apiHost = import.meta.env.VITE_API_HOST;

  console.log('API Host is ' + apiHost);

  const [test, setTest] = useState<{ test: string }[]>([]);

  const [version, setVersion] = useState<{version: string}[]>([]);

   useEffect(() => {
     fetch(apiHost + 'api/test')
         .then((response) => response.json())
         .then((data) => {
	    setTest(data);
            console.log(data);
         })
         .catch((err) => {
            console.log(err);
         });
   }, []);

   useEffect(() => {
     fetch(apiHost + 'api/version')
         .then((response) => response.json())
         .then((data) => {
	    setVersion(data);
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
	  {version.length === 0}
	  {version.length !== 0 && <p>{version[0].version}</p>}
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
