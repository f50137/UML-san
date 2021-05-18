class Lotto {

	private Zufallsgenerator generator;
	private int[] lottozahlen;

	public Lotto() {
		lottozahlen = new int[6];
		generator = new Zufallsgenerator();
	}
	

	//Ohne hilfe aus dem Internet
	public void sortiere4() {
		for(int i = 0; i < lottozahlen.length - 1; i++) {
			for(int j = i+1; j < lottozahlen.length;j++) {
				vergleichTausch(i,j);
			}
		}
	}
	
	//Mit hilfe des Internets
	public void sortiere3() {
		for(int i = 1; i < lottozahlen.length; i++) {
			for(int j = i; j > 0; j--) {
				if(lottozahlen[j] < lottozahlen[j-1]) tausche(j, j-1);
				else break;
			}
		}
	}

	//Version aus dem Internet
	public void sortiere2() {
		for(int i = 1; i < lottozahlen.length; i++) {
			int value = lottozahlen[i];
			int j;
			for(j = i; j > 0 && lottozahlen[j - 1] > value; j--) {
				lottozahlen[j] = lottozahlen[j - 1];
			}
			lottozahlen[j] = value;
		}
	}
	
	//Version 2 aus dem Internet
	public void sortiere() {
		boolean bOK;
		int value1;
		int value2;

		int k = -1;
		int i;
		value1 = lottozahlen[lottozahlen.length - 1];
		for(i = lottozahlen.length - 1; i > 0; i--) {
			value2 = lottozahlen[i - 1];
			if(value1 < value2) {
				lottozahlen[i - 1] = value1;
				lottozahlen[i] = value2;
				k = i;
			} else {
				value1 = value2;
			}
		}
		if(k < 0) return;

		
		for(i = 2; i < lottozahlen.length; i++) {
			value1 = lottozahlen[i];
			k = i;
			bOK = true;
			while(bOK) {
				value2 = lottozahlen[k - 1];
				if(value1 < value2) {
					lottozahlen[k] = value2;
					k--;
				} else {
					bOK = false;
				}
			}
			if(k < i) lottozahlen[k] = value1;
		}
	}

	public void tausche(int a, int b)  {
		int temp = lottozahlen[a];
		lottozahlen[a] = lottozahlen[b];
		lottozahlen[b] = temp;
	}

	public void vergleichTausch(int a, int b) {
		if(lottozahlen[a] > lottozahlen[b]) {
			tausche(a,b);
		}
	}

	public void ziehen() {
		for(int i = 0; i < lottozahlen.length; i++) {
			lottozahlen[i] = generator.ziehe();
		}
		sortiere();
	}

	public void reset() {
		for(int i = 0; i < lottozahlen.length; i++) {
			lottozahlen[i] = 0;
		}
	}

	public int maxIndex() {
		int index = 0;
		for(int i = 1; i < lottozahlen.length; i++) {
			if(lottozahlen[i] > lottozahlen[index])
				index = i;
		}
		return index;
	}

	public int maxWert() {
		return lottozahlen[maxIndex()];
	}

	public int minIndex() {
		int index = 0;
		for(int i = 0; i < lottozahlen.length; i++) {
			if(lottozahlen[i] < lottozahlen[index])
				index = i;
		}

		return index;
	}

	public int minWert() {
		return lottozahlen[minIndex()];
	}

	public boolean element(int n) {
		for(int elem : lottozahlen) {
			if(elem == n) {
				return true;
			}
		}
		return false;
	}

	public void zahlenAusgeben() {
		for(int elem : lottozahlen) {
			System.out.print(elem + ", ");
		}
		System.out.print("\n");
	}

	public int[] getZahlen() {
		return lottozahlen;
	}
}
