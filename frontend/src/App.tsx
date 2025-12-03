import { BrowserRouter, Route, Routes } from "react-router";
import Home from "@/pages/home";
import "./App.css";

function App() {
	return (
		<BrowserRouter>
			<Routes>
				<Route path="/" element={<Home />} index></Route>
			</Routes>
		</BrowserRouter>
	);
}

export default App;
