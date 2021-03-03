import React, { useState, useEffect } from 'react';
import { Container, Image, Form, Button, Row, Col } from 'react-bootstrap';
import axios from 'axios';

const EditTeamScreen = ({ match, history }) => {
	const [member, setMember] = useState([]);
	const [name, setName] = useState('');
	const [description, setDescription] = useState('');
	const [memberType, setMemberType] = useState('');

	const memberId = match.params.id;

	const submitHandler = async (e) => {
		e.preventDefault();

		const memberUpdate = {
			id: memberId,
			name: name === '' ? member.name : name,
			image: member.image,
			description: description === '' ? member.description : description,
		};

		await axios.patch('/admin/team/', memberUpdate, { withCredentials: true });
	};

	const deleteMember = async (e) => {
		e.preventDefault();
		if (window.confirm(`Are you sure you want to delete this house?`)) {
			await axios.delete(
				'/admin/team/',
				{ headers: { 'Content-type': 'application/json' }, data: member},
				{ withCredentials: true }
			);
		}
		history.push('/admin')
	};

	useEffect(() => {
		const fetchAMember = async () => {
			const { data } = await axios.get(`/team/${memberId}`);
			setMember(data);
			setName(data.name);
			setDescription(data.description);
			setMemberType(data.member_type);
		};
		fetchAMember();
	}, []);

	return (
		<Container className='py-3'>
			<Row>
				<Col>
					<Image
						src={member.image}
						style={{
							width: '100%',
							objectFit: 'cover',
						}}
					/>
				</Col>
				<Col>
					<Form onSubmit={submitHandler}>
						<Form.Group>
							<Form.Label>Name</Form.Label>
							<Form.Control
								type='text'
								placeholder={member.name}
								size='sm'
								value={name}
								onChange={(e) => setName(e.target.value)}
							/>
							<Form.Text>
								Current name is <strong>{member.name}</strong>
							</Form.Text>
						</Form.Group>
						<Form.Group>
							<Form.Label>Description</Form.Label>
							<Form.Control
								type='text'
								placeholder={member.description}
								size='sm'
								value={description}
								onChange={(e) => setDescription(e.target.value)}
							/>
							<Form.Text>
								Current name is <strong>{member.description}</strong>
							</Form.Text>
						</Form.Group>
						<Form.Group>
							<Form.Label>Member Type</Form.Label>
							<Form.Control
								type='text'
								placeholder={member.member_type}
								size='sm'
								value={memberType}
								onChange={(e) => setMemberType(e.target.value)}
							/>
							<Form.Text>
								Current name is <strong>{member.member_type}</strong>
							</Form.Text>
						</Form.Group>
						<Button variant='primary' type='submit' block size='lg'>
							<h1 style={{ color: 'white' }}>Submit</h1>
						</Button>
					</Form>
					<Button
						variant='danger'
						type='submit'
						block
						size='sm'
						className='my-4'
						onClick={deleteMember}
					>
						<h1 style={{ color: 'white' }}>Delete</h1>
					</Button>
				</Col>
			</Row>
		</Container>
	);
};

export default EditTeamScreen;
