import * as THREE from 'three';
import { OrbitControls } from 'three/addons/controls/OrbitControls.js';
import { SVGLoader } from 'three/addons/loaders/SVGLoader.js';
import { artist_points } from 'artist_points'

const scene = new THREE.Scene();
const camera = new THREE.PerspectiveCamera(75, window.innerWidth / window.innerHeight, 0.1, 1000);

const renderer = new THREE.WebGLRenderer();
renderer.setSize(window.innerWidth, window.innerHeight);
document.getElementById("three").appendChild(renderer.domElement);

camera.position.z = 3;
camera.position.y = 4;


const controls = new OrbitControls(camera, renderer.domElement);
controls.enableDamping = true;
controls.enablePan = false;


const v3Array = [];

for (const [artist, point] of Object.entries(artist_points)) {
	v3Array.push(new THREE.Vector3(
		(point[1] + 97) / 5.4,
		0,
		(point[0] - 39) / -4

	))
}


const artist_points_geo = new THREE.Points(
	new THREE.BufferGeometry().setFromPoints(v3Array),
	new THREE.PointsMaterial({
		color: 0x85FFFF,
		size: 0.02
	}));

scene.add(artist_points_geo);

window.STATE_OBJECTS = {};

function animate() {
	requestAnimationFrame(animate);

	controls.update();

	renderer.render(scene, camera);
}

animate();

const QUERY_ELEM = document.getElementById("query");
const RESULTS_ELEM = document.getElementById("results");

document.body.addEventListener("keydown", function (e) {
	if(isChildOf(e.target, QUERY_ELEM.parentElement)) return;

	document.getElementById("ui").style.opacity = 1;
	QUERY_ELEM.focus();
});

function isChildOf(child, parent) {
	while(child.parentElement) {
		if(child == parent) return true;
		child = child.parentElement;
	}
	return false;
}

const API_ROOT = "http://localhost:3020/api";


(function () {

	let searcher_timer = 0;
	QUERY_ELEM.addEventListener("change", function () {
		clearTimeout(searcher_timer);
		searcher_timer = setTimeout(function () {
			var xhr = new XMLHttpRequest();
			xhr.open("GET", API_ROOT + "/artists/search/" + encodeURIComponent(QUERY_ELEM.value));
			xhr.onload = function() {
				let results = JSON.parse(xhr.responseText);
				RESULTS_ELEM.textContent = "";
				for (const [id, name] of results) {
					let elm = document.createElement("div");
					elm.addEventListener("click", () => {
						showPathsFor(id, name);
					})
					elm.tabIndex = 0;
					elm.setAttribute("data-artist", id);
					elm.setAttribute("data-artist-name", name);
					elm.textContent = name;
					RESULTS_ELEM.appendChild(elm);
				}
			}
			xhr.send();
		}, 50);
	});
}
)();


RESULTS_ELEM.addEventListener("click", function(e) {
	const f = e.target;
	if(!f.getAttribute("data-artist")) return;

	showPathsFor(f.getAttribute("data-artist"), f.getAttribute("data-artist-name"));
});
QUERY_ELEM.addEventListener("keypress", function(e) {
	if(e.key == "Enter") {
		const f = RESULTS_ELEM.children[0]
		showPathsFor(f.getAttribute("data-artist"), f.getAttribute("data-artist-name"));
	}
});

RESULTS_ELEM.addEventListener("keypress", function(e) {
	const f = e.target;
	showPathsFor(f.getAttribute("data-artist"), f.getAttribute("data-artist-name"));
	e.stopPropagation();
});


let artist_selected_point;

function showPathsFor(artistId, artistName) {
	var xhr = new XMLHttpRequest();
		xhr.open("GET", API_ROOT + "/artists/" + artistId + "/paths");
		xhr.onload = function() {
			let results = JSON.parse(xhr.responseText);

			RESULTS_ELEM.textContent = "";
			if(artist_selected_point) scene.remove(artist_selected_point);

			if (artistId in artist_points) {
				const point = artist_points[artistId];
				const group = new THREE.Group();

				let line = new THREE.Points(
					new THREE.BufferGeometry().setFromPoints(range(0,1,100).map(x=>
						new THREE.Vector3(
							(point[1] + 97) / 5.4,
							x,
							(point[0] - 39) / -4
						)
					)),
					new THREE.PointsMaterial({
						color: 0xFFAF85,
						size: 0.01
					}));

				let cap = new THREE.Points(
					new THREE.BufferGeometry().setFromPoints([
						new THREE.Vector3(
							(point[1] + 97) / 5.4,
							1,
							(point[0] - 39) / -4
					
						)
					]),
					new THREE.PointsMaterial({
						color: 0xFFAF85,
						size: 0.05
					}));

				group.add(line);
				group.add(cap);

				artist_selected_point = group;
				scene.add(artist_selected_point);
			}

			var values = Object.entries(results).sort((a,b)=>a[1].length - b[1].length);

			var home_state = values.shift()[0];

			document.getElementById("data-display").innerHTML = `<h2>${artistName}</h2>
			<div>Home State: ${home_state}</div>
			${
				values.slice(0, 7).map((x, i)=>`
				<div>
					${x[0]}: ${x[1].length - 2}
					<span>
					${
						x[1].filter(x=>x).map(x=>`<a>${x}</a>`).join("&#8594;")
					}
					</span>
				</div>
				`).join("")
			}
			`
		}
		xhr.send();
}

window.showPathsFor = showPathsFor;

function range(min, max, n) {
	let step = (max - min) / n;
	let f = [];
	for(let v = min; v <= max; v += step) f.push(v);
	return f;
}

// instantiate a loader
const loader = new SVGLoader();

// load a SVG resource
loader.load(
	// resource URL
	'/image/states.svg',
	// called when the resource is loaded
	function (data) {

		const paths = data.paths;
		const group = new THREE.Group();

		for (let i = 0; i < paths.length; i++) {

			const path = paths[i];

			const opacity = path.userData.style.fillOpacity || 0.01;

			const material = new THREE.MeshBasicMaterial({
				color: path.color,
				opacity: opacity,
				transparent: opacity != 1,
				side: THREE.DoubleSide,
				depthWrite: false
			});

			const shapes = SVGLoader.createShapes(path);

			for (let j = 0; j < shapes.length; j++) {

				const shape = shapes[j];
				const geometry = new THREE.ShapeGeometry(shape);
				const mesh = new THREE.Mesh(geometry, material);
				group.add(mesh);

				let title = path.userData.node.querySelector("title");
				if (title) STATE_OBJECTS[title.textContent.trim()] = mesh;
			}

		}

		scene.add(group);

		group.rotateX(Math.PI * 0.5);
		//group.rotateY(-Math.PI / 2);
		group.translateX(-5);
		group.translateY(-3);

	}
);