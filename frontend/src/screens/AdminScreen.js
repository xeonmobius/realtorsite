import React, { useState, useEffect } from 'react';
import { Row, Col, Container, Card, Button } from 'react-bootstrap';
import { LinkContainer } from 'react-router-bootstrap';
import axios from 'axios';

const AdminScreen = () => {
	const [auth, setAuth] = useState(false);
	const [houses, setHouses] = useState([]);
	const [team, setTeam] = useState([]);

	useEffect(() => {
		const requestAccess = async () => {
			const { data } = await axios.get('/auth', { withCredentials: true });
			setAuth(data);
		};
		const fetchHouses = async () => {
			const { data } = await axios.get('/houses');
			setHouses(data);
		};
		const fetchTeamData = async () => {
			const { data } = await axios.get('/team');
			setTeam(data);
		};

		fetchTeamData();
		fetchHouses();
		requestAccess();
	}, []);

	return (
		<Container className='py-3'>
			{!auth ? (
				<h1>Not authorized. Restricted Access</h1>
			) : (
				<>
					<Row>
						<Col>
							<h1>Edit Properties</h1>
						</Col>
						<Col>
							<Button variant='danger' size='sm'>
								<p style={{ color: 'white' }}>Add</p>
							</Button>
						</Col>
					</Row>
					<Row>
						{houses.map((house) => (
							<LinkContainer
								to={`/admin/edit/properties/${house.id}`}
								style={{ cursor: 'pointer' }}
								key={house.id}
							>
								<Col sm={12} md={6} lg={4} xl={3}>
									<Card className='my-2 mx-2' style={{ height: '90%' }}>
										<Card.Img
											variant='top'
											src={house.image}
											style={{
												width: '100%',
												height: '40vh',
												objectFit: 'cover',
											}}
										/>
										<Card.Body>
											<Card.Title>{house.street_name}</Card.Title>
										</Card.Body>
									</Card>
								</Col>
							</LinkContainer>
						))}
					</Row>
					<Row>
						<Col>
							<h1>Edit Team</h1>
						</Col>
						<Col>
							<Button variant='danger' size='sm'>
								<p style={{ color: 'white' }}>Add</p>
							</Button>
						</Col>
					</Row>
					<Row>
						{team.map((member) => (
							<LinkContainer
								to={`/admin/edit/team/${member.id}`}
								style={{ cursor: 'pointer' }}
								key={member.id}
							>
								<Col sm={12} md={6} lg={4} xl={3} >
									<Card
										className='my-2 mx-2'
										style={{
											width: '100%',
											height: '100%',
										}}
									>
										<Card.Img
											variant='top'
											src={member.image}
											style={{
												width: '100%',
												height: '40vh',
												objectFit: 'cover',
											}}
										/>
										<Card.Body>
											<Card.Title>{member.name}</Card.Title>
										</Card.Body>
									</Card>
								</Col>
							</LinkContainer>
						))}
					</Row>
				</>
			)}
		</Container>
	);
};

export default AdminScreen;
