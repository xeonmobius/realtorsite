import React from 'react';
import { Container, Image, Row, Col } from 'react-bootstrap';
import houseData from '../data/houses';

const PropertiesScreen = () => {
	return (
		<Container className='py-3'>
			<h1>{houseData[0].streetName}</h1>
			<Image
				src={houseData[0].image}
				style={{
					width: '100%',
					height: '100%',
					objectFit: 'cover',
				}}
			/>
            <h2 className='py-3'>
				$ {houseData[0].price}
			</h2>
			<h5>
				Lorem ipsum dolor sit amet, consectetur adipiscing elit. Maecenas vel
				dui eget risus porta dignissim ac nec mi. Pellentesque habitant morbi
				tristique senectus et netus et malesuada fames ac turpis egestas.
				Pellentesque sed tempus magna. In in est faucibus, pulvinar tortor ut,
				posuere erat. Sed ac dolor scelerisque, ultrices mi non, consectetur
				purus. Nunc ac faucibus leo. Nunc non semper enim. Phasellus feugiat
				elit sed nulla pharetra, sed gravida turpis feugiat. Donec eu imperdiet
				tellus, vitae ultrices nisi. Cras pellentesque velit lacus, et elementum
				sem mattis ac. Maecenas non aliquam felis, at feugiat nisl. Phasellus
				quam erat, semper ac leo in, feugiat efficitur erat. In mattis efficitur
				efficitur. Cras sed maximus mauris. Interdum et malesuada fames ac ante
				ipsum primis in faucibus. Pellentesque nec tristique erat.
			</h5>
		</Container>
	);
};

export default PropertiesScreen;
