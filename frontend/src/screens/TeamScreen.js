import React, {useState, useEffect} from 'react';
import { Container, Col, Row, Card, Image, Button } from 'react-bootstrap';
import teamData from '../data/team.js';
import { LinkContainer } from 'react-router-bootstrap';
import axios from 'axios';

const TeamScreen = () => {
	const [team, setTeam] = useState([]);
	
	useEffect(() => {
		const fetchTeamData = async () => {
			const {data} = await axios.get('/team');
			console.log(data.team)
			setTeam(data.team);
		}
		fetchTeamData();
	}, [])

	return (
		<>
			<Container className='py-3'>
				<h1>The Team</h1>
				<Row>
					{team.map((member, index) => (
						<>
							{member.type === 'main' && (
								<Row  className='py-3' fluid="true">
									<Col>
										<Image src={member.image} alt={member.name} fluid />
									</Col>
									<Col>
										<p>{member.description}</p>
									</Col>
								</Row>
							)}
							{member.type === 'member' && (
								<Col sm={12} md={6} lg={4} xl={3}>
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
											<Card.Text>{member.description}</Card.Text>
										</Card.Body>
									</Card>
								</Col>
							)}
						</>
					))}
				</Row>
			</Container>
			<Container className='py-3'>
				<LinkContainer to='/contact'>
					<Button size='lg' block variant='primary'>
						<h1 style={{ color: 'white' }}>Contact Us</h1>
					</Button>
				</LinkContainer>
			</Container>
		</>
	);
};

export default TeamScreen;
