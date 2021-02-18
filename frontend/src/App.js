import Navigationbar from './components/Navigationbar';
import HomeScreen from './screens/HomeScreen';
import PropertiesListScreen from './screens/PropertiesListScreen';
import TeamScreen from './screens/TeamScreen';
import PropertiesScreen from './screens/PropertiesScreen';
import ContactUsScreen from './screens/ContactUsScreen';
import { BrowserRouter as Router, Route, Switch } from 'react-router-dom';
import './App.css';

function App() {
	return (
		<Router>
			<Navigationbar />
			<main>
				<Switch>
					<Route path='/' component={HomeScreen} exact />
					<Route path='/properties' component={PropertiesListScreen} exact />
					<Route path='/propertiesHouse' component={PropertiesScreen} />
					<Route path='/team' component={TeamScreen} />
					<Route path='/contact' component={ContactUsScreen} />
				</Switch>
			</main>
		</Router>
	);
}

export default App;
