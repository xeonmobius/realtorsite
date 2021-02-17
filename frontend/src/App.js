import Navigationbar from './components/Navigationbar';
import HomeScreen from './screens/HomeScreen';
import PropertiesScreen from './screens/PropertiesScreen';
import './App.css';

function App() {
	return (
		<>
			<Navigationbar />
			<main>
				<HomeScreen />
			</main>
		</>
	);
}

export default App;
