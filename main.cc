#include <vector>
#include <random>
#include <iostream>
#include <fstream>
#include <sstream>
#include <algorithm>
#include <csignal>

using namespace std;

inline std::mt19937_64& global_random_engine()
{
	static std::random_device rdev;
	static std::mt19937_64 eng(rdev());
	return eng;
}

inline float canonical()
{
	return std::generate_canonical<float, 16>(global_random_engine());
}

constexpr size_t g_random_numbers_size = 32768;
vector<float> g_random_numbers;

void canonical_fast_initialize()
{
	g_random_numbers.clear();
	g_random_numbers.reserve(g_random_numbers_size);
	for (size_t i = 0; i < g_random_numbers_size; ++i)
		g_random_numbers.push_back(canonical());
}

inline float canonical_fast()
{
	static size_t i = 0;
	i = (i + 1) % g_random_numbers_size;
	return g_random_numbers[i];
}

vector<int> g_vmin, g_vmax;
vector<vector<float>> g_values;

bool parse_commandline(int argc, char* argv[], string& inputfile, string& outputfile, char& delim)
{
	if (argc < 2) {
		cerr << "no input file" << endl;
		return false;
	}

	inputfile = string(argv[1]);

	if (argc >= 3) outputfile = string(argv[2]);
	else outputfile = string("result_") + inputfile;

	if (argc >= 4) delim = argv[3][0];
	else delim = ',';

	return true;
}

bool parse_file(string filename, char delim)
{
	g_vmin.clear();
	g_vmax.clear();
	g_values.clear();

	ifstream file(filename);
	if (!file.good()) {
		cerr << "cannot open file " << filename << endl;
		return false;
	}
	string s1, s2;
	if (getline(file, s1)) {
		istringstream line(s1);
		while (getline(line, s2, delim)) {
			g_vmin.push_back(stoi(s2));
		}
	}
	if (g_vmin.empty()) {
		cerr << "cannot read the first line of the file" << endl;
		return false;
	}
	if (getline(file, s1)) {
		istringstream line(s1);
		while (getline(line, s2, delim)) {
			g_vmax.push_back(stoi(s2));
		}
	}
	if (g_vmin.size() != g_vmax.size()) {
		cerr << "there is " << g_vmin.size() << " min bound elements and " << g_vmax.size() << " max bound elements" << endl;
		return false;
	}

	int line_number = 2;
	while (getline(file, s1)) {
		line_number++;
		istringstream line(s1);
		g_values.push_back(vector<float>());
		while (getline(line, s2, delim)) {
			g_values.back().push_back(stoi(s2));
		}
		if (g_vmin.size() != g_values.back().size()) {
			cerr << "on line " << line_number << " of the file there is " << g_values.back().size() << " values but " << g_vmin.size() << " bounds" << endl;
			return false;
		}
	}
	return true;
}

void count(vector<int>& x, const vector<vector<float>>& values)
{
	fill(x.begin(), x.end(), 0);

	// occupation of each workshop
	for (size_t i = 0; i < values.size(); ++i) {
		size_t k = distance(values[i].begin(), min_element(values[i].begin(), values[i].end()));
		x[k]++;
	}

	for (size_t i = 0; i < x.size(); ++i) {
		if (x[i] < g_vmin[i])
			x[i] -= g_vmin[i]; // negative value for a lack
		else if (x[i] > g_vmax[i])
			x[i] -= g_vmax[i]; // positive value for an overage
		else
			x[i] = 0; // null value if in range
	}
}

vector<int> solve()
{
	float factor = 1e-3;
	vector<vector<float>> copy(g_values);
	vector<int> cnt(g_vmin.size());

	for (size_t i = 0; i < copy.size(); ++i) {
		for (size_t j = 0; j < copy[i].size(); ++j) {
			copy[i][j] += canonical() - 0.5;
		}
	}
	count(cnt, copy);

	while (any_of(cnt.begin(), cnt.end(), [](float x){return x != 0.0;})) {

		for (size_t i = 0; i < copy.size(); ++i) {
			for (size_t j = 0; j < g_vmin.size(); ++j) {
				copy[i][j] += factor * cnt[j] * canonical_fast();
			}
		}

		count(cnt, copy);
	}

	vector<int> results(g_vmin.size());
	for (size_t i = 0; i < copy.size(); ++i)
		results[i] = distance(copy[i].begin(), min_element(copy[i].begin(), copy[i].end()));

	return results;
}

bool run = true;
void leave(int)
{
	run = false;
}

vector<int> search_solution()
{
	int best_score = -1;
	vector<int> best_results;

	canonical_fast_initialize();

	int iteration = 0;
	vector<int> results;

	//#pragma omp parallel
	while (run && iteration < 20) {
		cout << iteration++ << "     \r" << flush;

		results = solve();

		int score = 0;
		for (size_t i = 0; i < g_values.size(); ++i) {
			score += pow(g_values[i][results[i]], 2);
		}

		//#pragma omp critical
		if (score < best_score || best_score == -1) {
			best_score = score;
			best_results = results;
			cout << "score : " << score << endl;
		}
	}

	return best_results;
}

bool write_output(string filename, const vector<int>& results)
{
	ofstream out(filename);
	if (!out.good()) {
		cerr << "cannot write output file" << endl;
		return false;
	}

	for (size_t i = 0; i < g_vmin.size(); ++i) {
		out << g_vmin[i];
		if (i < g_vmin.size() - 1)
			out << ',';
	}
	out << '\n';
	for (size_t i = 0; i < g_vmax.size(); ++i) {
		out << g_vmax[i];
		if (i < g_vmax.size() - 1)
			out << ',';
	}
	out << '\n';
	for (size_t i = 0; i < g_values.size(); ++i) {
		for (size_t j = 0; j < g_values[i].size(); ++j) {
			out << g_values[i][j] << ',';
		}
		out << results[i] << '\n';
	}
	out.close();
	return true;
}

int main(int argc, char* argv[])
{
	string inputfile, outputfile;
	char delim;

	signal(SIGINT, leave);

	if (!parse_commandline(argc, argv, inputfile, outputfile, delim)) return 1;
	if (!parse_file(inputfile, delim)) return 1;

	vector<int> results = search_solution();

	cout << "SAVING RESULTS..." << endl;
	if (!write_output(outputfile, results)) return 1;


	for (int i = 0; i < (int)g_values[0].size(); ++i) {
		size_t sum = 0;
		for (size_t j = 0; j < results.size(); ++j)
			if (g_values[j][results[j]] == i)
				sum++;
		cout << "#" << i << " choice : " << sum << " students (" << double(1000*sum/g_values.size())/10.0 << "%)." << endl;
	}

	cout << "for a total of " << g_values.size() << " students." << endl;

	for (size_t i = 0; i < g_values[0].size(); ++i) {
		size_t sum = 0;
		for (size_t j = 0; j < results.size(); ++j)
			if (results[j] == (int)i)
				sum++;
		cout << "workshop #" << i << " : " << g_vmin[i] << "<=" << sum << "<=" << g_vmax[i] << endl;
	}

	return 0;
}
