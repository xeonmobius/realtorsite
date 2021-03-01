import { BrowserRouter as Router, Route, Switch } from 'react-router-dom';
import Navigationbar from './components/Navigationbar';
import HomeScreen from './screens/HomeScreen';
import PropertiesListScreen from './screens/PropertiesListScreen';
import TeamScreen from './screens/TeamScreen';
import PropertiesScreen from './screens/PropertiesScreen';
import ContactUsScreen from './screens/ContactUsScreen';
import LoginScreen from './screens/LoginScreen';
import AdminScreen from './screens/AdminScreen';
import './App.css';

function App() {
	return (
		<Router>
			<Navigationbar />
			<main>
				<Switch>
					<Route path='/' component={HomeScreen} exact />
					<Route path='/properties' component={PropertiesListScreen} exact />
					<Route path='/properties/:id' component={PropertiesScreen}/>
					<Route path='/team' component={TeamScreen} />
					<Route path='/contact' component={ContactUsScreen} />
					<Route path='/admin/login' component={LoginScreen} />
					<Route path='/admin/' component={AdminScreen} />
				</Switch>
			</main>
		</Router>
	);
}

export default App;
