import React, { useState, useEffect } from 'react';
import { Container, Image, Form, Button, Row, Col } from 'react-bootstrap';
import axios from 'axios';

const EditScreen = ({ match, history }) => {
	const [house, setHouse] = useState([]);
	const [streetName, setStreetName] = useState('');
	const [description, setDescription] = useState('');
	const [longDescription, setLongDescription] = useState('');
	const [residentType, setResidentType] = useState('');
	const [price, setPrice] = useState('');
	const [image, setImage] = useState('');

	const reader = new FileReader();

	const houseId = match.params.id;

	const onImageUpload = (e) => {
		reader.readAsDataURL(e);

		reader.onload = function () {
			setImage(reader.result);
		};
	};

	const submitHandler = async (e) => {
		e.preventDefault();

		if (houseId === 'add') {
			const house = {
				id: "",
				street_name: streetName,
				image: image,
				description: description,
				long_description: longDescription,
				resident_type: residentType,
				price: price,
			};
			await axios.post('/admin/house/', house, {
				withCredentials: true,
			});
		} else {
			const houseUpdate = {
				id: houseId,
				street_name: streetName === '' ? house.street_name : streetName,
				image: image === '' ? house.image : image,
				description: description === '' ? house.description : description,
				long_description:
					longDescription === '' ? house.long_description : longDescription,
				resident_type: residentType === '' ? house.resident_type : residentType,
				price: price === '' ? house.price : price,
			};
			await axios.patch('/admin/house/', houseUpdate, {
				withCredentials: true,
			});
		}
	};

	const deleteHouse = async (e) => {
		e.preventDefault();
		if (window.confirm(`Are you sure you want to delete this house?`)) {
			await axios.delete(
				'/admin/house/',
				{ headers: { 'Content-type': 'application/json' }, data: house },
				{ withCredentials: true }
			);
		}
		history.push('/admin');
	};

	useEffect(() => {
		const fetchAHouse = async () => {
			const { data } = await axios.get(`/houses/${houseId}`);
			setHouse(data);
		};
		if (houseId === 'add') {
		} else {
			fetchAHouse();
		}
	}, []);

	return (
		<Container className='py-3'>
			<Row>
				<Col>
					<Image
						src={image !== '' ? image : house.image}
						style={{
							width: '100%',
							objectFit: 'cover',
						}}
					/>
					<Form.Group>
						<Form.Label>Image</Form.Label>
						<Form.Control
							type='file'
							size='sm'
							onChange={(e) => onImageUpload(e.target.files[0])}
						/>
					</Form.Group>
				</Col>
				<Col>
					<Form onSubmit={submitHandler}>
						<Form.Group>
							<Form.Label>Street Name</Form.Label>
							<Form.Control
								type='text'
								placeholder={house.street_name}
								size='sm'
								value={streetName}
								onChange={(e) => setStreetName(e.target.value)}
							/>
							<Form.Text>
								Current Address is <strong>{house.street_name}</strong>
							</Form.Text>
						</Form.Group>
						<Form.Group>
							<Form.Label>Description</Form.Label>
							<Form.Control
								type='text'
								placeholder={house.description}
								size='sm'
								value={description}
								onChange={(e) => setDescription(e.target.value)}
							/>
							<Form.Text>
								Current Description is <strong>{house.description}</strong>
							</Form.Text>
						</Form.Group>
						<Form.Group>
							<Form.Label>Long Description</Form.Label>
							<Form.Control
								type='text'
								placeholder={house.long_description}
								size='sm'
								value={longDescription}
								onChange={(e) => setLongDescription(e.target.value)}
							/>
							<Form.Text className='text'>
								Current Description is <strong>{house.long_description}</strong>
							</Form.Text>
						</Form.Group>
						<Form.Group>
							<Form.Label>Resident Type</Form.Label>
							<Form.Control
								type='text'
								placeholder={house.resident_type}
								size='sm'
								value={residentType}
								onChange={(e) => setResidentType(e.target.value)}
							/>
							<Form.Text className='text'>
								Current type is <strong>{house.resident_type}</strong>
							</Form.Text>
						</Form.Group>
						<Form.Group>
							<Form.Label>Price</Form.Label>
							<Form.Control
								type='number'
								placeholder={house.price}
								size='sm'
								value={price}
								onChange={(e) => setPrice(e.target.value)}
							/>
							<Form.Text className='text'>
								Current price is $ <strong>{house.price}</strong>
							</Form.Text>
						</Form.Group>
						<Button variant='primary' type='submit' block size='lg'>
							<h1 style={{ color: 'white' }}>Submit</h1>
						</Button>
					</Form>
					{houseId !== 'add' && (
						<Button
							variant='danger'
							type='submit'
							block
							size='sm'
							className='my-4'
							onClick={deleteHouse}
						>
							<h1 style={{ color: 'white' }}>Delete</h1>
						</Button>
					)}
				</Col>
			</Row>
		</Container>
	);
};

export default EditScreen;
