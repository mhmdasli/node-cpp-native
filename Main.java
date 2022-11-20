public class Main {
    public static boolean isPrime(Integer num) {
        if (num <= 1) {
            return false;
        }
        for (Integer i = 2; i <= num / 2; i++) {
            if (num % i == 0) {
                return false;
            }
        }
        return true;
    }

    public static Integer findPrime(Integer num) {
        Integer largestPrime = 0;
        for (Integer j = 2; j <= num; j++) {
            if (isPrime(j)) {
                largestPrime = j;
            }
        }
        return largestPrime;
    }

    public static void main(String[] args) {
        long time = System.currentTimeMillis();
        System.out.println("prime = " + findPrime(500000));
        System.out.println("elapsed: " + (System.currentTimeMillis() - time)/1000L);
    }
}