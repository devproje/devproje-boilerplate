import Home from "@/pages/home";
import { BrowserRouter, Route, Routes } from "react-router";
import GeneralLayout from "@/components/layout/general-layout";

import "./App.scss";

function App() {
	return (
		<BrowserRouter>
			<Routes>
				<Route element={<GeneralLayout />}>
					<Route path="/" element={<Home />} index></Route>
				</Route>
			</Routes>
		</BrowserRouter>
	);
}

export default App;
