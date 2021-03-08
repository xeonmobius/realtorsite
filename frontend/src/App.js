import { BrowserRouter as Router, Route, Switch } from 'react-router-dom';
import Navigationbar from './components/Navigationbar';
import HomeScreen from './screens/HomeScreen';
import PropertiesListScreen from './screens/PropertiesListScreen';
import TeamScreen from './screens/TeamScreen';
import PropertiesScreen from './screens/PropertiesScreen';
import ContactUsScreen from './screens/ContactUsScreen';
import LoginScreen from './screens/LoginScreen';
import AdminScreen from './screens/AdminScreen';
import EditScreen from './screens/EditScreen';
import EditTeamScreen from './screens/EditTeamScreen';
import BlogScreen from './screens/BlogScreen';
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
					<Route path='/blog' component={BlogScreen} exact />
					<Route path='/admin/login' component={LoginScreen} />
					<Route path='/admin/' component={AdminScreen} exact/>
					<Route path='/admin/edit/properties/:id' component={EditScreen} />
					<Route path='/admin/edit/team/:id' component={EditTeamScreen} exact />

				</Switch>
			</main>
		</Router>
	);
}

export default App;
