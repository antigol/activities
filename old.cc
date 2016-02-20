#include <fstream>
#include <sstream>
#include <iostream>
#include <vector>
#include <algorithm>
#include <random>
#include <signal.h>

using namespace std;

// Compte pour chaque atelier combien il manque ou il y a en trop d'élèves
void count(vector<float>& ret, const vector<vector<float>>& eleves, const vector<int>& vmin, const vector<int>& vmax)
{
	// compte les participant pour chaque ateliers
	fill(ret.begin(), ret.end(), 0.0);
	for (size_t i = 0; i < eleves.size(); ++i) {
		size_t k = distance(eleves[i].begin(), min_element(eleves[i].begin(), eleves[i].end()));
		ret[k]++;
	}
	
	// regarde si il y a trop ou pas assez de participants
	for (size_t i = 0; i < ret.size(); ++i) {
		if (ret[i] < vmin[i])
			ret[i] -= vmin[i]; // négatif pour un manque
		else if (ret[i] > vmax[i])
			ret[i] -= vmax[i]; // positif pour un surplus
		else
			ret[i] = 0; // nul si le nombre de participants est dans [vmin, vmax]
	}
}

inline std::default_random_engine& global_random_engine()
{
	static std::random_device rdev;
	static std::default_random_engine eng(rdev()); 
	return eng;
}

// retourne un nombre entre 0 et 1 (non compris)
inline float canonical()
{
	return std::generate_canonical<float, 16>(global_random_engine());
}

inline double uniform(double a, double b)
{
	static std::uniform_real_distribution<> d{};
	using parm_t = decltype(d)::param_type;
	return d(global_random_engine(), parm_t{a, b});
}

inline unsigned long xorshf96(void)
{
	static unsigned long x=123456789, y=362436069, z=521288629;
	unsigned long t;
	x ^= x << 16;
	x ^= x >> 5;
	x ^= x << 1;

	t = x;
	x = y;
	y = z;
	z = t ^ x ^ y;
	return z;
}

inline float canonical_fast()
{
	double x = double(xorshf96()) / double(numeric_limits<unsigned long>::max());
	return float(x);
}

constexpr size_t random_numbers_size = 32768; // 128ko de nombres aléatoires
vector<float> random_numbers;
void canonical_fast2_init()
{
	random_numbers.reserve(random_numbers_size);
	for (size_t i = 0; i < random_numbers_size; ++i)
		random_numbers.push_back(canonical());
}

inline float canonical_fast2() // deux fois plus rapide que canonical_fast
{
	static size_t i = 0;
	i = (i + 1) % random_numbers_size;
	return random_numbers[i];
}

void solve(vector<int>& ret, vector<vector<float>> eleves, const vector<int>& vmin, const vector<int>& vmax)
{
	// factor petit => long temps de calcul mais meilleur résultat
	//float factor = pow(10.0, uniform(-4.0, -2.0));
	float factor = 1e-3;
	
	// modifie un peu tout les chiffres pour peu etre chopper un meilleur résultat
	for (size_t i = 0; i < eleves.size(); ++i) {
		for (size_t j = 0; j < eleves[i].size(); ++j) {
			eleves[i][j] += canonical() - 0.5;
		}
	}
	vector<float> c(vmin.size());
	count(c, eleves, vmin, vmax);

	// tant que tout les atelier ne sont pas corectement remplis on fait la boucle suivante
	// apellée environ 2'000 fois avec factor=1e-3 et 200'000 fois avec factor=1e-5
	while (any_of(c.begin(), c.end(), [](float x){return x != 0.0;})) {
		for (size_t i = 0; i < c.size(); ++i)
			c[i] *= factor;
		
		for (size_t i = 0; i < eleves.size(); ++i) {
			for (size_t j = 0; j < eleves[i].size(); ++j) {
				eleves[i][j] += c[j] * canonical_fast2(); // modifie les envie dans le sens du surplus/manque des ateliers
			}
		}
		// puis on regarde ce que ca change
		count(c, eleves, vmin, vmax);
	}
	
	// on obtient finalement un atelier par élève
	for (size_t i = 0; i < eleves.size(); ++i)
		ret[i] = distance(eleves[i].begin(), min_element(eleves[i].begin(), eleves[i].end()));
}

bool run = true;
void leave(int sig)
{
	run = false;
}

int main(int argc, char* argv[])
{
	signal(SIGINT, leave);
	
	canonical_fast2_init();
	
	// on parse le fichier
	if (argc != 2) return 1;
	vector<int> vmin, vmax;
	vector<vector<float>> eleves;
	
	ifstream file(argv[1]);
	if (!file.good())
		return 1;
	string s1, s2;
	if (getline(file, s1)) {
		istringstream line(s1);
		while (getline(line, s2, ',')) {
			vmin.push_back(stoi(s2));
		}
	}
	if (getline(file, s1)) {
		istringstream line(s1);
		while (getline(line, s2, ',')) {
			vmax.push_back(stoi(s2));
		}
	}
	while (getline(file, s1)) {
		istringstream line(s1);
		eleves.push_back(vector<float>());
		while (getline(line, s2, ',')) {
			eleves.back().push_back(stoi(s2));
		}
	}
	
	int best_score = 1e9;
	vector<int> best_ret(eleves.size());
	int i = 0;

	// cherche des solutions et on garde la meilleure
	#pragma omp parallel 
	while (run) {
		cout << i++ << "     \r" << flush;
		
		// Lance solve
		vector<int> ret(eleves.size());
		solve(ret, eleves, vmin, vmax);
		
		// calcul le score
		int sum = 0, score = 0;
		for (size_t i = 0; i < eleves.size(); ++i) {
			sum += eleves[i][ret[i]];
			score += pow(eleves[i][ret[i]], 2);
		}
		
		// affiche si c'est meilleur
		#pragma omp critical
		if (score < best_score) {
			best_score = score;
			best_ret = ret;
			cout << "total : " << sum << endl;
			cout << "score : " << score << endl;
		}
	}
	
	cout << "SAVING RESULTS..." << endl;
	ofstream out(string("results_") + string(argv[1]));
	if (!out.good())
		return 1;
	
	for (size_t i = 0; i < vmin.size(); ++i) {
		out << vmin[i];
		if (i < vmin.size() - 1)
			out << ',';
	}
	out << endl;
	for (size_t i = 0; i < vmax.size(); ++i) {
		out << vmax[i];
		if (i < vmax.size() - 1)
			out << ',';
	}
	out << endl;
	for (size_t i = 0; i < eleves.size(); ++i) {
		for (size_t j = 0; j < eleves[i].size(); ++j) {
			out << eleves[i][j] << ',';
		}
		out << best_ret[i] << endl;
	}
	
	for (int i = 0; i < (int)eleves[0].size(); ++i) {
		size_t sum = 0;
		for (size_t j = 0; j < best_ret.size(); ++j)
			if (eleves[j][best_ret[j]] == i)
				sum++;
		cout << "#" << i << " choice : " << sum << " students (" << double(1000*sum/eleves.size())/10.0 << "%)." << endl;
	}
	
	cout << "for a total of " << eleves.size() << " students." << endl;
	
	for (size_t i = 0; i < eleves[0].size(); ++i) {
		size_t sum = 0;
		for (size_t j = 0; j < best_ret.size(); ++j)
			if (best_ret[j] == (int)i)
				sum++;
		cout << "workshop #" << i << " : " << vmin[i] << "<=" << sum << "<=" << vmax[i] << endl;
	}
	
	return 0;
}
